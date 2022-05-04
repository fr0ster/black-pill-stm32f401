#![no_main]
#![no_std]

use cortex_m::asm::delay;
#[allow(unused)]
use panic_halt;

use cortex_m_rt as rt;
use stm32f4xx_hal as hal;
use hal::prelude::*;


const SLOW_DELAY: u32 = 3000000;
const LONG_DELAY: u32 = 12000000;

#[rt::entry]
fn main() -> ! {
    if let Some(peripherals) = hal::stm32::Peripherals::take() {
        let gpioc = peripherals.GPIOC.split();

        let mut led = gpioc.pc13.into_push_pull_output();

        let gpioa = peripherals.GPIOA.split();
        let button = gpioa.pa0.into_pull_up_input();

        loop {
            let mut delay_time = SLOW_DELAY;
            if button.is_high().unwrap() {
                delay_time = LONG_DELAY;
            }
            led.set_low().unwrap();
            delay(delay_time);
            led.set_high().unwrap();
            delay(delay_time);
        }
    }
    loop {}
}