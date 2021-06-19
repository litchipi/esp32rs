use esp32_hal::{
    target,
    gpio::*,
    i2c::*,
};

pub const CORE_HZ: u32 = 40_000_000;

#[allow(unused_imports)]
use crate::blinky::BlinkyAlgo;
#[allow(unused_imports)]
use crate::oled_simple::OledSimpleAlgo;

pub type WorkAlgo = OledSimpleAlgo;

/*      Oled Simple Algorithm           */
type OledRstPin = Gpio16<Output<PushPull>>;
pub fn get_oled_i2c_rst() -> OledRstPin{
    let pins = target::Peripherals::take().unwrap().GPIO.split();
    pins.gpio16.into_push_pull_output()
}

pub type OledI2cInstance = target::I2C0;
pub fn get_oled_i2c_instance() -> OledI2cInstance{
    target::Peripherals::take().unwrap().I2C0
}

type OledI2cPins = Pins<Gpio15<Unknown>, Gpio4<Unknown>>;
pub fn get_oled_i2c_pins() -> OledI2cPins{
    let pins = target::Peripherals::take().unwrap().GPIO.split();
    Pins {
        sda: pins.gpio15,
        scl: pins.gpio4,
    }
}



