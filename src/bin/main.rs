//! Quick Start for STM32 H743
#![no_std]
#![no_main]
use defmt;
use defmt_rtt as _;
use panic_probe as _;

use stm32h7xx_hal::{gpio, prelude::*};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32h7xx_hal::pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();

    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    let ccdr = rcc.sysclk(120.MHz()).freeze(pwrcfg, &dp.SYSCFG);

    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);
    let mut led_green = gpiob.pb0.into_push_pull_output();
    let mut led_red = gpiob.pb14.into_push_pull_output();

    // Also, we can set pin speed.
    led_red.set_speed(gpio::Speed::High);

    let mut delay = cp.SYST.delay(ccdr.clocks);
    defmt::info!("This is a Info.");
    defmt::warn!("This is a warn");
    defmt::error!("This is a Error");
    defmt::println!("This is just print");
    loop {
        // light up Yellow first
        defmt::println!("Yellow");
        led_green.set_high();
        delay.delay_ms(500u32);
        //  light up Red next
        defmt::println!("Red");
        led_red.set_high();
        delay.delay_ms(500u16);

        // led out after 500ms
        led_green.set_low();
        led_red.set_low();
        delay.delay_ms(500u16);
    }
}
