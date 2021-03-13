#![no_main]
#![no_std]

use panic_halt as _;

use nrf52840_hal as hal;

use hal::{pac::{CorePeripherals, Peripherals},
        prelude::*,
        gpio::Level,
        delay::Delay,                
        };

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    
    let p = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    let port0 = hal::gpio::p0::Parts::new(p.P0);

    let mut led = port0.p0_06.into_push_pull_output(Level::High); // blue LED, active low
    
    let mut delay = Delay::new(core.SYST);
    
    loop {       
        
        //set_high turns the LED off
        
        led.set_low().unwrap();
        delay.delay_ms(500_u32);              
        led.set_high().unwrap(); 
        delay.delay_ms(1000_u32);              
        
    }
    
}