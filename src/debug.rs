#[allow(dead_code)]
use esp32_hal::target;
use esp32_hal::prelude::*;
use esp32_hal::gpio::*;

pub struct DebugHandler{
    led_pin: Gpio4<Output<PushPull>>,
}

impl DebugHandler{
    pub fn new() -> DebugHandler{
        let dp = target::Peripherals::take().unwrap();
        let pins = dp.GPIO.split();

        DebugHandler{
            led_pin: pins.gpio4.into_push_pull_output(),
        }
    }

    pub fn debug_indic(&mut self, val: bool){
        if val { 
            self.led_pin.set_high().unwrap()
        } else {
            self.led_pin.set_low().unwrap()
        }
    }
}
