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
    gpio::{Analog, Gpiod, Input, Pin, U},
    pac::{self, ADC3},
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
    pd14_pin: Pin<Gpiod, U<14>, Analog>,
    leds: LedArray,
    delay: Delay,
    adc3: Adc<ADC3>,
    usb_serial: SerialPortType<'a>,
    usb_device: UsbDevType<'a>,
}

#[entry]
fn main() -> ! {
    let device_periphs = pac::Peripherals::take().unwrap();
    let core_periphs = cortex_m::Peripherals::take().unwrap();
    let mut reset_and_clock_control = device_periphs.RCC.constrain();
    let mut flash = device_periphs.FLASH.constrain();
    let clocks = get_clocks(reset_and_clock_control.cfgr, &mut flash);
    let mut delay = Delay::new(core_periphs.SYST, clocks);
    let gpioa = device_periphs.GPIOA.split(&mut reset_and_clock_control.ahb);
    let mut gpiod = device_periphs.GPIOD.split(&mut reset_and_clock_control.ahb);
    let gpioe = device_periphs.GPIOE.split(&mut reset_and_clock_control.ahb);
    let mut leds = get_leds(gpioe);

    let button_d3 = gpiod
        .pd3
        .into_floating_input(&mut gpiod.moder, &mut gpiod.pupdr);

    let pd14_pin = gpiod.pd14.into_analog(&mut gpiod.moder, &mut gpiod.pupdr);

    let adc3 = get_adc(
        Adc3Arg {
            adc3: device_periphs.ADC3,
            adc3_4: device_periphs.ADC3_4,
            ahb: reset_and_clock_control.ahb,
        },
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
        pd14_pin,
        leds,
        delay,
        adc3,
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
        let button_state = app.button_d3.is_high().unwrap();

        if button_state {
            controller_state.a = true;

            let to_send = controller_state.to_string();
            _ = app.usb_serial.write(to_send.as_bytes());
            _ = app.usb_serial.flush().is_ok();

            app.leds[5].on().ok();

            app.delay.delay_us(100u32);
        } else {
            controller_state.a = false;

            let to_send = controller_state.to_string();
            _ = app.usb_serial.write(to_send.as_bytes());
            _ = app.usb_serial.flush().is_ok();

            app.leds[5].off().ok();

            app.delay.delay_us(100u32);
        }
    }

    let adc1_in1_data: u16 = app
        .adc3
        .read(&mut app.pd14_pin)
        .expect("Error reading adc3.");

    let adc_val_32 = adc1_in1_data as f32;

    let _scaled = adc_val_32 / 4095_f32;

    // for curr in 0..8 {
    //     let current = (curr as f32) / 8_f32;
    //     if _scaled > current {
    //         leds[curr].on().ok();
    //     } else {
    //         leds[curr].off().ok();
    //     }
    // }

    if !app.usb_device.poll(&mut [&mut app.usb_serial]) {
        // leds[0].on().ok();
        return;
    } else {
        app.leds[0].on().ok();
    }
}
