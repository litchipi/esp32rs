use esp32_hal::{
    clock_control::{self, sleep, CPUSource, ClockControl},
    dport::Split,
    i2c::{Pins, Error, I2C},
    prelude::*,
    target::Peripherals,
    timer::Timer,
    gpio::*,
    target,
    gpio::*,
    i2c::*,
};

#[allow(unused_imports)]
use crate::blinky::BlinkyAlgo;
#[allow(unused_imports)]
use crate::oled_simple::OledSimpleAlgo;

pub type WorkAlgo = OledSimpleAlgo;



/*      Blinky Algorithm                */
pub type BlinkyLedPin = Gpio2<Output<PushPull>>;



/*      Oled Simple Algorithm           */
pub type OledI2cInstance = target::I2C0;

#[macro_export]
macro_rules! get_oled_pin {
    [ i2c_rst, $pins:expr ] => { $pins.gpio16 };
    [ i2c_inst, $dp:expr ] => { $dp.I2C0 };
    [ i2c_pins, $pins:expr ] => { 
        Pins {
            sda: $pins.gpio4,
            scl: $pins.gpio15,
        }
    };
}
