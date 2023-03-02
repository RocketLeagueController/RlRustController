#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::fmt::Write;
use cortex_m::asm::delay;
use cortex_m::iprintln;
use cortex_m_rt::entry;
use cortex_m_semihosting::{hio, hprintln};
use fugit::ExtU32;
use panic_halt as _;
use stm32f3xx_hal::gpio::{Alternate, Gpioa, Input, Output, Pin, PushPull, U};
use stm32f3xx_hal::usb::{DmPin, DpPin, Peripheral, UsbBus};
use stm32f3xx_hal::{adc, prelude::*};
use stm32f3xx_hal::{pac, prelude::*};
use usb_device::prelude::*;
use usbd_human_interface_device::device::keyboard::NKROBootKeyboardInterface;
use usbd_human_interface_device::page::Keyboard;
use usbd_human_interface_device::prelude::UsbHidClassBuilder;
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
        .pclk2(24.MHz())
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

    // BluePill board has a pull-up resistor on the D+ line.
    // Pull the D+ pin down to send a RESET condition to the USB bus.
    // This forced reset is needed only for development, without it host
    // will not reset your device when you upload new firmware.
    let mut usb_dp = gpioa
        .pa12
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    usb_dp.set_low().ok();
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

    //let serial = SerialPort::new(&usb_bus);

    // let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
    //     .manufacturer("Fake company")
    //     .product("Serial port")
    //     .serial_number("TEST")
    //     .device_class(USB_CLASS_CDC)
    //     .build();

    let mut keyboard = UsbHidClassBuilder::new()
        .add_interface(NKROBootKeyboardInterface::default_config())
        .build(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x1209, 0x0001))
        .manufacturer("usbd-human-interface-device")
        .product("NKRO Keyboard")
        .serial_number("TEST")
        .build();

    let pd3_pin = gpiod
        .pd3
        .into_floating_input(&mut gpiod.moder, &mut gpiod.pupdr)
        .downgrade()
        .downgrade();

    let pd14_pin = gpiod.pd14.into_analog(&mut gpiod.moder, &mut gpiod.pupdr);


    delay(clocks.sysclk().0 / 100);
    // let mut adc3 = adc::Adc::adc3(
    //     device_periphs.ADC3, // The ADC we are going to control
    //     // The following is only needed to make sure the clock signal for the ADC is set up
    //     // correctly.
    //     &mut device_periphs.ADC3_4,
    //     &mut reset_and_clock_control.ahb,
    //     adc::CkMode::default(),
    //     clocks,
    // );

    led2.set_high();
    led3.set_high();
    led4.set_high();
    led5.set_high();
    led6.set_high();

    //let mut tick_timer = timer.count_down();
    //tick_timer.start(1.millis());

    loop {
        // let keys = if pin.is_high().unwrap() {
        //         [Keyboard::A]
        //     } else {
        //         [Keyboard::NoEventIndicated]
        //};

        let keys = [Keyboard::A];

        keyboard.interface().write_report(keys).ok();

        //tick once per ms/at 1kHz
        //if tick_timer.wait().is_ok() {
        keyboard.interface().tick().unwrap();
        //}

        if usb_dev.poll(&mut [&mut keyboard]) {
            match keyboard.interface().read_report() {
                // Ok(l) => {
                //     update_leds(l);
                // }
                _ => {}
            }
        }
    }
}
