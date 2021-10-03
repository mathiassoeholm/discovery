#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let period = 100;

    let mut turn_on_index = 2;
    let mut turn_off_index = 0;
    let mut turn_off_time = 50_u16;
    let mut turn_on_time = 100_u16;
    let mut time = 0_u16;

    leds[0].on().ok();
    leds[1].on().ok();

    loop {
        if time >= turn_off_time {
            leds[turn_off_index].off().ok();
            turn_off_time += period;
            turn_off_index = (turn_off_index + 1) % 8;
        }

        if time >= turn_on_time {
            leds[turn_on_index].on().ok();
            turn_on_index = (turn_on_index + 1) % 8;
            turn_on_time += period;
        }

        delay.delay_ms(50_u16);
        time += 50_u16;
    }
}
