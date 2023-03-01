//! Initialization code

#![no_std]

pub use panic_itm; // panic handler

pub use cortex_m_rt::entry;

use stm32f3_discovery::{button::UserButton, stm32f3xx_hal::gpio::{Pin, marker::{Gpio, Index}, Input, Gpioa, U, Ux, Gpiox}};
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

pub struct InitStruct
{
    pub delay: Delay,
    pub leds: LedArray,
    pub button_a0 : UserButton,
    pub pd3_pin : Pin<Gpiox, Ux, Input>,
    // buttons
}

pub fn init() -> InitStruct {
    let device_periphs = pac::Peripherals::take().unwrap();
    let mut reset_and_clock_control = device_periphs.RCC.constrain();

    let core_periphs = cortex_m::Peripherals::take().unwrap();
    let mut flash = device_periphs.FLASH.constrain();
    let clocks = reset_and_clock_control.cfgr.freeze(&mut flash.acr);
    let delay = Delay::new(core_periphs.SYST, clocks);

    let mut gpioa = device_periphs.GPIOA.split(&mut reset_and_clock_control.ahb);
    let mut gpiod = device_periphs.GPIOD.split(&mut reset_and_clock_control.ahb);

    let button_a0 = UserButton::new(gpioa.pa0, &mut gpioa.moder, &mut gpioa.pupdr);


    let pd3_pin = gpiod.pd3.into_floating_input(
        & mut gpiod.moder,
        &mut gpiod.pupdr).downgrade().downgrade();

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

    InitStruct { 
        delay: delay, 
        leds: leds.into_array(),
        button_a0: button_a0,
        pd3_pin: pd3_pin,
    }
}
