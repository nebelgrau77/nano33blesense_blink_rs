#![no_main]
#![no_std]

use panic_halt as _;

use nrf52840_hal as hal;

use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::gpio::Level;
use hal::delay::Delay;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    
    let p = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    let port0 = hal::gpio::p0::Parts::new(p.P0);

    let mut red = port0.p0_24.into_push_pull_output(Level::Low);
    let mut green = port0.p0_16.into_push_pull_output(Level::Low);    
    let mut blue = port0.p0_06.into_push_pull_output(Level::Low);
    
    let mut delay = Delay::new(core.SYST);
    
    loop {       
        
        //set_high actually shuts the LED off

        blue.set_low().unwrap();
        delay.delay_ms(500_u32);              
        blue.set_high().unwrap(); 
        delay.delay_ms(500_u32);              
        
        red.set_low().unwrap();
        delay.delay_ms(750_u32);
        red.set_high().unwrap();
        delay.delay_ms(250_u32);

        green.set_low().unwrap();
        delay.delay_ms(250_u32);
        green.set_high().unwrap();
        delay.delay_ms(750_u32);
        
    }
    
}