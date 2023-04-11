#![deny(unsafe_code)]
#![no_main]
#![no_std]

use controller::ControllerState;
pub use panic_itm; // panic handler

pub use cortex_m_rt::entry;

use cortex_m::prelude::{_embedded_hal_adc_OneShot, _embedded_hal_blocking_delay_DelayMs};

use source::leds::Leds;
use stm32_usbd::UsbBus;
use stm32f3xx_hal::{
    adc,
    delay::Delay,
    flash::Parts,
    pac::{self},
    prelude::{
        _embedded_hal_blocking_delay_DelayUs, _embedded_hal_digital_InputPin,
        _embedded_hal_digital_OutputPin, _stm32f3xx_hal_flash_FlashExt,
        _stm32f3xx_hal_gpio_GpioExt,
    },
    rcc::{Clocks, Rcc, RccExt, CFGR},
    time::rate::Megahertz,
    usb::Peripheral, gpio::{gpioe, Output, Ux, PushPull, self, Pin},
};
use switch_hal::{ActiveHigh, Switch, OutputSwitch};
use usb_device::prelude::{UsbDeviceBuilder, UsbVidPid};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

mod controller;

fn get_leds(mut gpioe : gpioe::Parts) -> [Switch<Pin<gpio::Gpioe, Ux, Output<PushPull>>, ActiveHigh>; 8] {
    let leds = Leds::new(
        gpioe.pe8,
        gpioe.pe9,
        gpioe.pe10,
        gpioe.pe11,
        gpioe.pe12,
        gpioe.pe13,
        gpioe.pe14,
        gpioe.pe15,
        &mut gpioe.moder,
        &mut gpioe.otyper,
    )
    .into_array();

    return leds;
}

fn get_clocks(
    cfgr: CFGR,
    flash : &mut Parts
) -> Clocks {

    let clocks = cfgr
        .use_hse(Megahertz::new(8))
        .sysclk(Megahertz::new(48))
        .pclk1(Megahertz::new(24))
        .pclk2(Megahertz::new(24))
        .freeze(&mut flash.acr);

    assert!(clocks.usbclk_valid());

    return clocks;
}

#[entry]
fn main() -> ! {
    let mut device_periphs = pac::Peripherals::take().unwrap();

    let core_periphs = cortex_m::Peripherals::take().unwrap();

    let mut reset_and_clock_control = device_periphs.RCC.constrain();

    let mut flash = device_periphs.FLASH.constrain();

    let clocks = 
        get_clocks(
            reset_and_clock_control.cfgr,
            &mut flash);

    let mut delay = Delay::new(core_periphs.SYST, clocks);

    let mut gpioa = device_periphs.GPIOA.split(&mut reset_and_clock_control.ahb);
    let mut gpiod = device_periphs.GPIOD.split(&mut reset_and_clock_control.ahb);

    //let button_a0 = UserButton::new(gpioa.pa0, &mut gpioa.moder, &mut gpioa.pupdr);

    let pd3_pin = gpiod
        .pd3
        .into_floating_input(&mut gpiod.moder, &mut gpiod.pupdr)
        .downgrade()
        .downgrade();

    let pd14_pin = gpiod.pd14.into_analog(&mut gpiod.moder, &mut gpiod.pupdr);

    // initialize user leds
    let mut gpioe = device_periphs.GPIOE.split(&mut reset_and_clock_control.ahb);

    let mut leds = get_leds(gpioe);

    let mut adc3 = adc::Adc::adc3(
        device_periphs.ADC3, // The ADC we are going to control
        // The following is only needed to make sure the clock signal for the ADC is set up
        // correctly.
        &mut device_periphs.ADC3_4,
        &mut reset_and_clock_control.ahb,
        adc::ClockMode::default(),
        clocks,
    );

    let button_d3 = pd3_pin;
    let mut analog_input_d14 = pd14_pin;

    // F3 Discovery board has a pull-up resistor on the D+ line.
    // Pull the D+ pin down to send a RESET condition to the USB bus.
    // This forced reset is needed only for development, without it host
    // will not reset your device when you upload new firmware.

    let mut usb_dp = gpioa
        .pa12
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    usb_dp.set_low().ok();

    delay.delay_ms(10_u16);

    let usb_dm =
        gpioa
            .pa11
            .into_af14_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);
    let usb_dp = usb_dp.into_af14_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);

    let usb = Peripheral {
        usb: device_periphs.USB,
        pin_dm: usb_dm,
        pin_dp: usb_dp,
    };

    let usb_bus = UsbBus::new(usb);

    let mut serial = SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(USB_CLASS_CDC)
        .build();

    leds[0].off().ok();

    let mut controller_state = ControllerState::new();

    let mut nb_iter = 0u64;

    loop {
        nb_iter += 1;
        // TODO : use clock

        if nb_iter % 1000 == 0 {
            let button_state = button_d3.is_high().unwrap();

            if button_state {
                controller_state.a = true;

                let to_send = controller_state.to_string();
                _ = serial.write(to_send.as_bytes());
                _ = serial.flush().is_ok();

                leds[5].on().ok();

                delay.delay_us(100u32);
            } else {
                controller_state.a = false;

                let to_send = controller_state.to_string();
                _ = serial.write(to_send.as_bytes());
                _ = serial.flush().is_ok();

                leds[5].off().ok();

                delay.delay_us(100u32);
            }
        }

        let adc1_in1_data: u16 = adc3
            .read(&mut analog_input_d14)
            .expect("Error reading adc3.");
        let adc_val_32 = adc1_in1_data as f32;

        let scaled = adc_val_32 / 4095_f32;

        // for curr in 0..8 {
        //     let current = (curr as f32) / 8_f32;
        //     if scaled >  current {
        //         leds[curr].on().ok();
        //     } else {
        //         leds[curr].off().ok();
        //     }
        // }

        if !usb_dev.poll(&mut [&mut serial]) {
            // leds[0].on().ok();
            continue;
        } else {
            leds[0].on().ok();
        }
    }
}
