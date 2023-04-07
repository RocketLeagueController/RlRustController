#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{
    entry, stm32f3xx_hal::{prelude::{_embedded_hal_digital_InputPin, _embedded_hal_adc_OneShot, _stm32f3xx_hal_flash_FlashExt, _stm32f3xx_hal_gpio_GpioExt, _embedded_hal_digital_OutputPin}, adc::{self, Adc}, rcc::RccExt, self, usb::Peripheral}, switch_hal::InputSwitch, Delay,
    DelayMs, LedArray, OutputSwitch, pac::{ADC3, self}, Leds,
};
use stm32_usbd::UsbBus;
use usb_device::prelude::{UsbDeviceBuilder, UsbVidPid};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

//use stm32f3xx_hal::usb::{Peripheral, UsbBus};


#[entry]
fn main() -> ! {
    //let (mut delay, mut leds): (Delay, LedArray) = aux5::init();
    //let init_struct = aux5::init();


    let mut device_periphs = pac::Peripherals::take().unwrap();
    let mut reset_and_clock_control = device_periphs.RCC.constrain();

    let core_periphs = cortex_m::Peripherals::take().unwrap();
    let mut flash = device_periphs.FLASH.constrain();
    let clocks = reset_and_clock_control.cfgr.freeze(&mut flash.acr);
    let mut delay = Delay::new(core_periphs.SYST, clocks);

    let mut gpioa = device_periphs.GPIOA.split(&mut reset_and_clock_control.ahb);
    let mut gpiod = device_periphs.GPIOD.split(&mut reset_and_clock_control.ahb);

    //let button_a0 = UserButton::new(gpioa.pa0, &mut gpioa.moder, &mut gpioa.pupdr);

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

    let mut leds: LedArray = Leds::new(
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
    ).into_array();

    let mut adc3 = adc::Adc::adc3(
        device_periphs.ADC3, // The ADC we are going to control
        // The following is only needed to make sure the clock signal for the ADC is set up
        // correctly.
        &mut device_periphs.ADC3_4,
        &mut reset_and_clock_control.ahb,
        adc::CkMode::default(),
        clocks,
    );

    // let mut leds = init_struct.leds;
    // let mut delay = init_struct.delay;
    // //let button_a0 = init_struct.button_a0;
    let button_d3 = pd3_pin;
    let mut analog_input_d14 = pd14_pin;
    // let mut adc3: Adc<ADC3> = init_struct.adc3;

    // F3 Discovery board has a pull-up resistor on the D+ line.
    // Pull the D+ pin down to send a RESET condition to the USB bus.
    // This forced reset is needed only for development, without it host
    // will not reset your device when you upload new firmware.
    let mut usb_dp = gpioa
        .pa12
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    usb_dp.set_low().ok();

    // TODO
    //delay(clocks.sysclk().0 / 100);

    let usb_dm = gpioa.pa11.into_af14_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);
    let usb_dp = usb_dp.into_af14_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);

    let usb = Peripheral {
        usb: device_periphs.USB,
        pin_dm: usb_dm,
        pin_dp: usb_dp,
    };

    let usb_bus = UsbBus::new(usb);

    let mut serial = SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .product("Serial port")
        .device_class(USB_CLASS_CDC)
        .build();

    loop {
        let button_state = button_d3.is_high().unwrap();

        if button_state {
            leds[5].on().ok();
        } else {
            leds[5].off().ok();
        }

        // let button_state = button_a0.is_active().unwrap();

        // if button_state {
        //     leds[0].on().ok();
        // } else {
        //     leds[0].off().ok();
        // }

        let adc1_in1_data: u16 = adc3.read(&mut analog_input_d14).expect("Error reading adc3.");
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

        delay.delay_ms(10_u16);
    }
    //}
}
