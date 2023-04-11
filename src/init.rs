use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;

use stm32f3xx_hal::{
    adc::{self, Adc},
    delay::Delay,
    flash::Parts,
    gpio::{self, gpioa, gpioe, Alternate, Output, Pin, PushPull, Ux, U},
    pac::{ADC3, ADC3_4, USB},
    prelude::_embedded_hal_digital_OutputPin,
    rcc::{Clocks, AHB, CFGR},
    time::rate::Megahertz,
    usb::Peripheral,
};

use switch_hal::{ActiveHigh, Switch};

use crate::leds::Leds;

pub type LedArray = [Switch<Pin<gpio::Gpioe, Ux, Output<PushPull>>, ActiveHigh>; 8];

pub type UsbPeriph = Peripheral<
    Pin<gpio::Gpioa, U<11>, Alternate<PushPull, 14>>,
    Pin<gpio::Gpioa, U<12>, Alternate<PushPull, 14>>,
>;

pub fn get_leds(mut gpioe: gpioe::Parts) -> LedArray {
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

pub struct Adc3Arg {
    pub adc3: ADC3,
    pub adc3_4: ADC3_4,
    pub ahb: AHB,
}

pub fn get_adc(mut arg: Adc3Arg, clocks: Clocks) -> Adc<ADC3> {
    let adc3 = adc::Adc::adc3(
        arg.adc3, // The ADC we are going to control
        // The following is only needed to make sure the clock signal for the ADC is set up
        // correctly.
        &mut arg.adc3_4,
        &mut arg.ahb,
        adc::ClockMode::default(),
        clocks,
    );

    return adc3;
}

pub fn get_clocks(cfgr: CFGR, flash: &mut Parts) -> Clocks {
    let clocks = cfgr
        .use_hse(Megahertz::new(8))
        .sysclk(Megahertz::new(48))
        .pclk1(Megahertz::new(24))
        .pclk2(Megahertz::new(24))
        .freeze(&mut flash.acr);

    assert!(clocks.usbclk_valid());

    return clocks;
}

pub fn get_usb_init(mut gpioa: gpioa::Parts, delay: &mut Delay, usb: USB) -> UsbPeriph {
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

    let usb_periph = Peripheral {
        usb: usb,
        pin_dm: usb_dm,
        pin_dp: usb_dp,
    };

    // usb::Peripheral<dyn DmPin, dyn DpPin>

    usb_periph
}
