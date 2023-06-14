#![deny(unsafe_code)]
#![no_main]
#![no_std]

use controller::ControllerState;
pub use panic_itm; // panic handler

pub use cortex_m_rt::entry;

use cortex_m::prelude::_embedded_hal_adc_OneShot;

use stm32_usbd::UsbBus;

use stm32f3xx_hal::{
    adc::Adc,
    delay::Delay,
    gpio::{Analog, Gpiod, Gpiob, Input, Pin, U, Ux},
    pac::{self, ADC3, ADC4},
    prelude::{
        _embedded_hal_blocking_delay_DelayUs, _embedded_hal_digital_InputPin,
        _stm32f3xx_hal_flash_FlashExt, _stm32f3xx_hal_gpio_GpioExt,
    },
    rcc::RccExt,
};

use source::init::*;
use switch_hal::OutputSwitch;
use usb_device::{
    class_prelude::UsbBusAllocator,
    prelude::{UsbDevice, UsbDeviceBuilder, UsbVidPid},
};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

mod controller;

pub type SerialPortType<'a> = SerialPort<'a, UsbBus<UsbPeriph>>;

type UsbDevType<'a> = UsbDevice<'a, UsbBus<UsbPeriph>>;

struct App<'a> {
    button_d3: Pin<Gpiod, U<3>, Input>,
    button_d4: Pin<Gpiod, U<4>, Input>,
    button_d5: Pin<Gpiod, U<5>, Input>,
    button_d6: Pin<Gpiod, U<6>, Input>,
    button_d7: Pin<Gpiod, U<7>, Input>,
    button_d1: Pin<Gpiod, U<1>, Input>,
    button_d0: Pin<Gpiod, U<0>, Input>,
    button_d2: Pin<Gpiod, U<2>, Input>,

    button_b4: Pin<Gpiob, U<4>, Input>,
    button_b5: Pin<Gpiob, U<5>, Input>,

    pb15_pin: Pin<Gpiob, U<15>, Analog>,
    pd8_pin: Pin<Gpiod, U<8>, Analog>,
    pd9_pin: Pin<Gpiod, U<9>, Analog>,
    pd10_pin: Pin<Gpiod, U<10>, Analog>,
    pd11_pin: Pin<Gpiod, U<11>, Analog>,
    pd12_pin: Pin<Gpiod, U<12>, Analog>,
    pd13_pin: Pin<Gpiod, U<13>, Analog>,
    pd14_pin: Pin<Gpiod, U<14>, Analog>,

    leds: LedArray,
    delay: Delay,
    adc3: Adc<ADC3>,
    adc4: Adc<ADC4>,
    usb_serial: SerialPortType<'a>,
    usb_device: UsbDevType<'a>,
}

#[entry]
fn main() -> ! {
    let mut device_periphs = pac::Peripherals::take().unwrap();
    let core_periphs = cortex_m::Peripherals::take().unwrap();
    let mut reset_and_clock_control = device_periphs.RCC.constrain();
    let mut flash = device_periphs.FLASH.constrain();
    let clocks = get_clocks(reset_and_clock_control.cfgr, &mut flash);
    let mut delay = Delay::new(core_periphs.SYST, clocks);
    let gpioa = device_periphs.GPIOA.split(&mut reset_and_clock_control.ahb);
    let mut gpiod = device_periphs.GPIOD.split(&mut reset_and_clock_control.ahb);
    let mut gpioe = device_periphs.GPIOE.split(&mut reset_and_clock_control.ahb);
    let mut gpiob = device_periphs.GPIOB.split(&mut reset_and_clock_control.ahb);
    let mut leds = get_leds(gpioe);

    let button_d3 = gpiod
        .pd3
        .into_floating_input(&mut gpiod.moder, &mut gpiod.pupdr);

    let button_d4 = gpiod
        .pd4
        .into_floating_input(&mut gpiod.moder, &mut gpiod.pupdr);

    let button_d5 = gpiod
        .pd5
        .into_floating_input(&mut gpiod.moder, &mut gpiod.pupdr);

    let button_d6 = gpiod
        .pd6
        .into_floating_input(&mut gpiod.moder, &mut gpiod.pupdr);

    let button_d7 = gpiod
        .pd7
        .into_floating_input(&mut gpiod.moder, &mut gpiod.pupdr);

    let button_d1 = gpiod
        .pd1
        .into_floating_input(&mut gpiod.moder, &mut gpiod.pupdr);

    let button_d0 = gpiod
        .pd0
        .into_floating_input(&mut gpiod.moder, &mut gpiod.pupdr);

    let button_d2 = gpiod
        .pd2
        .into_floating_input(&mut gpiod.moder, &mut gpiod.pupdr);

    let button_b4 = gpiob
        .pb4
        .into_floating_input(&mut gpiob.moder, &mut gpiob.pupdr);

    let button_b5 = gpiob
        .pb5
        .into_floating_input(&mut gpiob.moder, &mut gpiob.pupdr);

    let pd8_pin = gpiod.pd8.into_analog(&mut gpiod.moder, &mut gpiod.pupdr);
    let pd9_pin = gpiod.pd9.into_analog(&mut gpiod.moder, &mut gpiod.pupdr);
    let pd10_pin = gpiod.pd10.into_analog(&mut gpiod.moder, &mut gpiod.pupdr);
    let pd11_pin = gpiod.pd11.into_analog(&mut gpiod.moder, &mut gpiod.pupdr);
    let pd12_pin = gpiod.pd12.into_analog(&mut gpiod.moder, &mut gpiod.pupdr);
    let pd13_pin = gpiod.pd13.into_analog(&mut gpiod.moder, &mut gpiod.pupdr);
    let pd14_pin = gpiod.pd14.into_analog(&mut gpiod.moder, &mut gpiod.pupdr);
    let pb15_pin = gpiob.pb15.into_analog(&mut gpiob.moder, &mut gpiob.pupdr);

    let adc3 = get_adc3(
        device_periphs.ADC3,
        &mut device_periphs.ADC3_4,
        &mut reset_and_clock_control.ahb,
        clocks,
    );
    
    let adc4 = get_adc4(
        device_periphs.ADC4,
        &mut device_periphs.ADC3_4,
        &mut reset_and_clock_control.ahb,
        clocks,
    );

    let usb_peripheral: UsbPeriph = get_usb_init(gpioa, &mut delay, device_periphs.USB);
    let usb_bus: UsbBusAllocator<_> = UsbBus::new(usb_peripheral);
    let usb_serial: SerialPortType<'_> = SerialPort::new(&usb_bus);

    let usb_device = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(USB_CLASS_CDC)
        .build();

    leds[0].off().ok();

    let mut controller_state = ControllerState::new();
    let mut nb_iter = 0u64;

    let mut app = App {
        button_d3,
        button_d4,
        button_d5,
        button_d6,
        button_d7,
        button_d1,
        button_d0,
        button_d2,
        button_b4,
        button_b5,
        pb15_pin,
        pd8_pin,
        pd9_pin,
        pd10_pin,
        pd11_pin,
        pd12_pin,
        pd13_pin,
        pd14_pin,
        leds,
        delay,
        adc3,
        adc4,
        usb_serial,
        usb_device,
    };

    loop {
        run_main_loop_iter(&mut nb_iter, &mut controller_state, &mut app);
    }
}

fn run_main_loop_iter(nb_iter: &mut u64, controller_state: &mut ControllerState, app: &mut App) {
    *nb_iter += 1;
    // TODO : use clock

    if *nb_iter % 1000 == 0 {

        read_buttons_states(app, controller_state);
        read_joystick_states(app, controller_state);

        let to_send = controller_state.to_string();
        _ = app.usb_serial.write(to_send.as_bytes());
        _ = app.usb_serial.flush().is_ok();

        app.delay.delay_us(100u32);
    }
                                                                                                                                                                                                                                                                                                                                                                              
    // To debug ADC
    // let value = controller_state.other_value_1;
    // for curr in 0..8 {
    //     let current = (curr as f32) / 8_f32;
    //     if value >= current {
    //         app.leds[curr].on().ok();
    //     } else {
    //         app.leds[curr].off().ok();
    //     }
    // }

    // if !app.usb_device.poll(&mut [&mut app.usb_serial]) {
    //     // leds[0].on().ok();
    //     return;
    // } else {
    //     app.leds[0].on().ok();
    // }

}

fn read_adc_value(result: Result<u16, stm32f3xx_hal::nb::Error<()>>) -> f32 {
    let adc_value: u16 = result.expect("Error reading adc.");
    let adc_val_32 = adc_value as f32;
    let scaled_adb_value = adc_val_32 / 4095_f32;
    scaled_adb_value
}

fn read_joystick_states(app: &mut App, controller_state: &mut ControllerState) {
    let val = app.adc4.read(&mut app.pd8_pin);
    controller_state.left_thumb_x = read_adc_value(val);

    let val = app.adc4.read(&mut app.pd9_pin);
    controller_state.left_thumb_y = read_adc_value(val);

    let val = app.adc3.read(&mut app.pd10_pin);
    controller_state.right_thumb_x = read_adc_value(val);

    let val = app.adc3.read(&mut app.pd11_pin);
    controller_state.right_thumb_y = read_adc_value(val);

    let val = app.adc3.read(&mut app.pd12_pin);
    controller_state.left_trigger = read_adc_value(val);

    let val = app.adc3.read(&mut app.pd13_pin);
    controller_state.right_trigger = read_adc_value(val);

    let val = app.adc4.read(&mut app.pd14_pin);
    controller_state.other_value_0 = read_adc_value(val);

    let val = app.adc4.read(&mut app.pb15_pin);
    controller_state.other_value_1 = read_adc_value(val);
}

fn read_buttons_states(app: &mut App, controller_state: &mut ControllerState) {
    controller_state.a = app.button_d3.is_high().unwrap();
    controller_state.b = app.button_d4.is_high().unwrap();
    controller_state.x = app.button_d5.is_high().unwrap();
    controller_state.y = app.button_d6.is_high().unwrap();
    controller_state.left_shoulder = app.button_d1.is_high().unwrap();
    controller_state.right_shoulder = app.button_d7.is_high().unwrap();
    controller_state.left_thumb = app.button_d0.is_high().unwrap();
    controller_state.right_thumb = app.button_d2.is_high().unwrap();
    controller_state.start = app.button_b4.is_high().unwrap();
    controller_state.back = app.button_b5.is_high().unwrap();

    if controller_state.start {
        app.leds[2].on().ok();
    } else {
        app.leds[2].off().ok();
    }

    if controller_state.back {
        app.leds[3].on().ok();
    } else {
        app.leds[3].off().ok();
    }

    
}
