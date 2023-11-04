//! Quick Start for STM32 H743
#![no_std]
#![no_main]
use defmt::{error, info, println, warn};
use defmt_rtt as _;
use panic_probe as _;

use stm32h7xx_hal::prelude::*;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32h7xx_hal::stm32::pac::take().unwrap();
    let rcc = dp.RCC.constrain();

    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    let ccdr = rcc.sysclk(120.MHz()).freeze(pwrcfg, &dp.SYSCFG);

    loop {}
}
