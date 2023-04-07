//! Initialization code

#![no_std]

pub use panic_itm; // panic handler

pub use cortex_m_rt::entry;

use stm32f3_discovery::{
    button::UserButton,
    stm32f3xx_hal::{gpio::{
        marker::{Gpio, Index},
        Analog, Gpioa, Gpiox, Input, Pin, Ux, U, Gpiod,
    }, adc::{self, Adc}, pac::{ADC1, ADC3, Peripherals}, rcc::Rcc},
};

pub use stm32f3_discovery::{leds::Leds, stm32f3xx_hal, switch_hal};

pub use switch_hal::{ActiveHigh, OutputSwitch, Switch, ToggleableOutputSwitch};

use stm32f3xx_hal::prelude::*;

pub use stm32f3xx_hal::{
    delay::Delay,
    gpio::{gpioe, Output, PushPull},
    hal::blocking::delay::DelayMs,
    pac,
};

pub type LedArray = [Switch<gpioe::PEx<Output<PushPull>>, ActiveHigh>; 8];

use usb_device::{prelude::{UsbDeviceBuilder, UsbVidPid}, class_prelude::UsbBusAllocator};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

//use stm32f3xx_hal::usb::{Peripheral, UsbBus};

pub struct InitStruct {
    pub delay: Delay,
    pub leds: LedArray,
    pub button_a0: UserButton,
    pub pd3_pin: Pin<Gpiox, Ux, Input>,
    pub pd14_pin: Pin<Gpiod, U<14>, Analog>,
    pub adc3: Adc<ADC3>,
    // pub reset_and_clock_control: Rcc,
    // pub device_periphs: Peripherals,
    // buttons
}

pub fn init() -> InitStruct {
    let mut device_periphs = pac::Peripherals::take().unwrap();
    let mut reset_and_clock_control = device_periphs.RCC.constrain();

    let core_periphs = cortex_m::Peripherals::take().unwrap();
    let mut flash = device_periphs.FLASH.constrain();
    let clocks = reset_and_clock_control.cfgr.freeze(&mut flash.acr);
    let delay = Delay::new(core_periphs.SYST, clocks);

    let mut gpioa = device_periphs.GPIOA.split(&mut reset_and_clock_control.ahb);
    let mut gpiod = device_periphs.GPIOD.split(&mut reset_and_clock_control.ahb);

    let button_a0 = UserButton::new(gpioa.pa0, &mut gpioa.moder, &mut gpioa.pupdr);

    let pd3_pin = gpiod
        .pd3
        .into_floating_input(&mut gpiod.moder, &mut gpiod.pupdr)
        .downgrade()
        .downgrade();

    let pd14_pin = gpiod
        .pd14
        .into_analog(&mut gpiod.moder, &mut gpiod.pupdr);

    // initialize user leds
    let mut gpioe = device_periphs.GPIOE.split(&mut reset_and_clock_control.ahb);

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
    );

    let mut adc3 = adc::Adc::adc3(
        device_periphs.ADC3, // The ADC we are going to control
        // The following is only needed to make sure the clock signal for the ADC is set up
        // correctly.
        &mut device_periphs.ADC3_4,
        &mut reset_and_clock_control.ahb,
        adc::CkMode::default(),
        clocks,
    );


    InitStruct {
        delay: delay,
        leds: leds.into_array(),
        button_a0: button_a0,
        pd3_pin: pd3_pin,
        pd14_pin: pd14_pin,
        adc3: adc3,
        // reset_and_clock_control,
        // device_periphs,
    }
}
