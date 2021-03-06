/*
Next step: Draw an image

use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
};

//TODO	OLed simple: Find a way to convert .png images to .raw at compile time
//TODO	Pass path to data in configuration of Cargo.toml
const raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("../data/images/rust.raw"), 64);
const img: Image<'static, ImageRaw<BinaryColor>> = Image::new(&raw, Point::new(32, 0));
img.draw(&mut display).unwrap();
display.flush().unwrap();

*/

use embedded_graphics::{
    fonts::{Font8x16, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    style::TextStyle,
};

use embedded_hal::blocking::i2c::{Write as I2CWrite, WriteRead};

use esp32_hal::{
    clock_control::{self, sleep, CPUSource, ClockControl},
    dport::Split,
    i2c::{Pins, Error, I2C},
    prelude::*,
    target::Peripherals,
    timer::Timer,
    gpio::*,
};

use ssd1306::{prelude::*, Builder};
use xtensa_lx::mutex::SpinLockMutex;

use heapless::String;
use ufmt::uwrite;

use crate::Algo;

use crate::get_oled_pin;
use crate::config::{OledI2cInstance, OledResetPin};

type OledDisplay = ssd1306::mode::GraphicsMode<I2cInterface<I2CWrapper>>;
type I2cController = I2C<OledI2cInstance>;

const TEXT_LINE_SIZE: usize = 15;

pub struct DisplayDriver {
    pub display: OledDisplay,
}

impl DisplayDriver{
    pub fn new(mut rst_pin: OledResetPin, i2c_handler: I2CWrapper) -> DisplayDriver {
        DisplayDriver {
            display: {
                let mut display: GraphicsMode<_> = Builder::new().connect_i2c(i2c_handler).into();
                rst_pin.set_low().unwrap();
                sleep(10.ms());
                rst_pin.set_high().unwrap();
                display.init().unwrap();
                display.clear();
                display.flush().unwrap();
                display
            },
      //      rst_pin
        }
    }

    pub fn draw_text(&mut self, text: String<TEXT_LINE_SIZE>, x: i32, y: i32) {
        Text::new(text.as_str(), Point::new(x, y))
            .into_styled(TextStyle::new(Font8x16, BinaryColor::On))
            .draw(&mut self.display)
            .unwrap();
        self.display.flush().unwrap();
    }
}




pub struct OledSimpleAlgo{
    i: usize,
    display: DisplayDriver,
}

impl Algo for OledSimpleAlgo{
    fn init() -> Self where Self: Sized{
        let dp = Peripherals::take().unwrap();
        let (mut dport, dport_clock_control) = dp.DPORT.split();

        // setup clocks & watchdog
        let mut clkcntrl = ClockControl::new(
            dp.RTCCNTL,
            dp.APB_CTRL,
            dport_clock_control,
            clock_control::XTAL_FREQUENCY_AUTO,
        ).unwrap();

        // set desired clock frequencies
        clkcntrl.set_cpu_frequencies(
                CPUSource::PLL,
                80.MHz(),
                CPUSource::PLL,
                240.MHz(),
                CPUSource::PLL,
                80.MHz(),
            ).unwrap();

        // disable RTC watchdog
        let (clkcntrl_config, mut watchdog) = clkcntrl.freeze().unwrap();
        watchdog.disable();

        // disable MST watchdogs
        let (.., mut watchdog0) = Timer::new(dp.TIMG0, clkcntrl_config);
        let (.., mut watchdog1) = Timer::new(dp.TIMG1, clkcntrl_config);
        watchdog0.disable();
        watchdog1.disable();

        let pins = dp.GPIO.split();
        let i2c_t = I2C::new(
            get_oled_pin!(i2c_inst, dp),
            get_oled_pin!(i2c_pins, pins),
            400_000,
            &mut dport,
        );
        let i2cw = I2CWrapper::new(i2c_t);


        OledSimpleAlgo{
            display : DisplayDriver::new(get_oled_pin!(i2c_rst, pins).into_push_pull_output(), i2cw),
            i: 0,
        }
    }

    fn loop_fct(&mut self) {
        if self.i > 9999 {
            panic!("too large number");
        }

        let mut text: String<TEXT_LINE_SIZE> = String::new();
        uwrite!(text, "Hello rust {}", self.i).unwrap();
        self.display.draw_text(text, 2, 28);
        self.i += 1;
        sleep(1.s());
        self.display.display.clear()
    }
}

pub struct I2CWrapper {
    i2c: I2cController,
}

impl I2CWrapper {
    pub fn new(i2c: I2cController) -> Self {
        Self { i2c }
    }
}

impl I2CWrite for I2CWrapper {
    type Error = Error;

    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.i2c.write(addr, bytes)
    }
}

impl WriteRead for I2CWrapper {
    type Error = Error;

    fn write_read(&mut self, address: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.i2c.write_read(address, bytes, buffer)
    }
}

