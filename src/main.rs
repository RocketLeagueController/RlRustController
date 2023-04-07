#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{
    entry, stm32f3xx_hal::{prelude::{_embedded_hal_digital_InputPin, _embedded_hal_adc_OneShot}, adc::{self, Adc}}, switch_hal::InputSwitch, Delay,
    DelayMs, LedArray, OutputSwitch, pac::ADC3,
};

#[entry]
fn main() -> ! {
    //let (mut delay, mut leds): (Delay, LedArray) = aux5::init();
    let init_struct = aux5::init();
    let mut leds = init_struct.leds;
    let mut delay = init_struct.delay;
    //let button_a0 = init_struct.button_a0;
    let button_d3 = init_struct.pd3_pin;
    let mut analog_input_d14 = init_struct.pd14_pin;
    let mut adc3: Adc<ADC3> = init_struct.adc3;

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
