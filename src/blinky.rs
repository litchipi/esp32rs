use esp32_hal::target;
use esp32_hal::prelude::*;
use esp32_hal::gpio::*;
use esp32_hal::clock_control::sleep;

use crate::Algo;
use crate::config::BlinkyLedPin;

pub struct BlinkyAlgo{
    led: BlinkyLedPin,
}

impl Algo for BlinkyAlgo{
    fn init() -> Self where Self: Sized{
        let dp = target::Peripherals::take().unwrap();
        let pins = dp.GPIO.split();

        BlinkyAlgo{
            led: pins.gpio2.into_push_pull_output(),
        }
    }

    fn loop_fct(&mut self){
        self.led.set_high().unwrap();
        sleep(1.s());
        self.led.set_low().unwrap();
        sleep(1.s());
    }
}
