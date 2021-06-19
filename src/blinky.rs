use esp32_hal::target;
use esp32_hal::prelude::*;
use esp32_hal::gpio::*;
use xtensa_lx::timer::delay;

use crate::Algo;
use crate::config::CORE_HZ;

pub struct BlinkyAlgo{
    led: Gpio2<Output<PushPull>>,
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
        delay(CORE_HZ);
        self.led.set_low().unwrap();
        delay(CORE_HZ);
    }
}
