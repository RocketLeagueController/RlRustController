#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m::asm::delay;
use cortex_m::iprintln;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f3xx_hal::gpio::{Alternate, Gpioa, Input, Output, Pin, PushPull, U};
use stm32f3xx_hal::usb::{DmPin, DpPin, Peripheral, UsbBus};
use stm32f3xx_hal::{adc, prelude::*};
use stm32f3xx_hal::{pac, prelude::*};
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(8.MHz())
        .sysclk(48.MHz())
        .pclk1(24.MHz())
        .freeze(&mut flash.acr);

    assert!(clocks.usbclk_valid());

    // Configure the on-board LED (PC13, green)
    let mut gpioc = dp.GPIOC.split(&mut rcc.ahb);
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let mut gpiod = dp.GPIOD.split(&mut rcc.ahb);
    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);

    let mut led0 = gpioe
        .pe8
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    let mut led1 = gpioe
        .pe9
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    let mut led2 = gpioe
        .pe10
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    let mut led3 = gpioe
        .pe11
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    let mut led4 = gpioe
        .pe12
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    let mut led5 = gpioe
        .pe13
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    let mut led6 = gpioe
        .pe14
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    let mut led7 = gpioe
        .pe15
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    led0.set_high(); // Turn off

    led2.set_high();

    led4.set_high();

    let mut itm = cp.unwrap().ITM;

    iprintln!(&mut itm.stim[0], "Start!");

    // BluePill board has a pull-up resistor on the D+ line.
    // Pull the D+ pin down to send a RESET condition to the USB bus.
    // This forced reset is needed only for development, without it host
    // will not reset your device when you upload new firmware.
    let mut usb_dp = gpioa
        .pa12
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    usb_dp.set_low();
    delay(clocks.sysclk().0 / 100);

    let dm_pin: Pin<Gpioa, U<11>, Alternate<PushPull, 14>> =
        gpioa
            .pa11
            .into_af_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);

    let dp_pin: Pin<Gpioa, U<12>, Alternate<PushPull, 14>> =
        usb_dp.into_af_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);

    let usb = Peripheral {
        usb: dp.USB,
        pin_dm: dm_pin,
        pin_dp: dp_pin,
    };

    let usb_bus = UsbBus::new(usb);

    let mut serial = SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(USB_CLASS_CDC)
        .build();

    let pd3_pin = gpiod
        .pd3
        .into_floating_input(&mut gpiod.moder, &mut gpiod.pupdr)
        .downgrade()
        .downgrade();

    let pd14_pin = gpiod.pd14.into_analog(&mut gpiod.moder, &mut gpiod.pupdr);


    // let mut adc3 = adc::Adc::adc3(
    //     device_periphs.ADC3, // The ADC we are going to control
    //     // The following is only needed to make sure the clock signal for the ADC is set up
    //     // correctly.
    //     &mut device_periphs.ADC3_4,
    //     &mut reset_and_clock_control.ahb,
    //     adc::CkMode::default(),
    //     clocks,
    // );

    loop {
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        let mut buf = [0u8; 64];

        match serial.read(&mut buf) {
            Ok(count) if count > 0 => {
                led1.set_low(); // Turn on

                // Echo back in upper case
                for c in buf[0..count].iter_mut() {
                    if 0x61 <= *c && *c <= 0x7a {
                        *c &= !0x20;
                    }
                }

                let mut write_offset = 0;
                while write_offset < count {
                    match serial.write(&buf[write_offset..count]) {
                        Ok(len) if len > 0 => {
                            write_offset += len;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        led0.set_low(); // Turn off
    }
}
