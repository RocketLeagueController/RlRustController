#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{Delay, DelayMs, LedArray, OutputSwitch, entry, switch_hal::InputSwitch, stm32f3xx_hal::prelude::_embedded_hal_digital_InputPin};

#[entry]
fn main() -> ! {
    //let (mut delay, mut leds): (Delay, LedArray) = aux5::init();
    let init_struct = aux5::init();
    let mut leds = init_struct.leds;
    let mut delay = init_struct.delay;
    let button_a0 = init_struct.button_a0;
    let button_d3 = init_struct.pd3_pin;
    let mut active = true;
    let mut button_was_active = false;

    let ms = 1000_u16;
    loop {
        //for curr in 0..8 {

            // if active {
            //     let next = (curr + 1) % 8;

            //     leds[next].on().ok();
            //     leds[curr].off().ok();
            // }

            //let button_state = button_a0.is_active().unwrap();

            let button_state = button_d3.is_high().unwrap();

            if button_state  {
                leds[5].on().ok();
            }
            else {
                leds[5].off().ok();
            }

            let button_state = button_a0.is_active().unwrap();

            if button_state  {
                leds[0].on().ok();
            }
            else {
                leds[0].off().ok();
            }

            delay.delay_ms(10_u16);
        }
    //}
}
