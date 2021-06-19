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
use crate::config::{OledResetPin, OledI2cInstance};
use crate::oled_simple::I2CWrapper;

use embedded_graphics::image::{Image, ImageRaw};

type OledDisplay = ssd1306::mode::GraphicsMode<I2cInterface<I2CWrapper>>;

pub struct DisplayDriver {
    pub display: OledDisplay,
    pub data: ImageRaw<'static, BinaryColor>,
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
            //TODO  Convert png image to raw at compile time
            data: ImageRaw::new(include_bytes!("../data/images/rust.raw"), 64, 64),
        }
    }

    //TODO  Try to change the color -> Does this screen support it ?
    pub fn draw_logo(&mut self, _color: [u8;3]) {
        let im = Image::new(&self.data, Point::new(32, 0));
        im.draw(&mut self.display).unwrap();
        self.display.flush().unwrap();
    }
}

#[derive(Clone)]
pub struct Color { 
    rgb: [u8;3],

    mode: [u8;3],
}

impl Color {
    pub fn new() -> Color{
        Color {
            rgb: [0, 0, 255],
            mode: [0, 1, 2],        // 0 : Nothing, 1: Increase, 2: Decrease
        }
    }

    pub fn next(&mut self) -> [u8;3] {
        for col in 0..self.rgb.len(){
            if self.mode[col] == 0{
                continue;
            } else if self.mode[col] == 1{
                self.rgb[col] += 1;
                if self.rgb[col] == 255{
                    self.mode[col] += 1;
                    self.mode[(col+1)%3] = 0;
                }
            } else if self.mode[col] == 2{
                self.rgb[col] -= 1;
                if self.rgb[col] == 0{
                    self.mode[col] = 0;
                    self.mode[(col+1)%3] = 1;
                }
            }
        }

        self.rgb
    }
}

pub struct OledLogoAlgo{
    display: DisplayDriver,
    color: Color,
}

impl Algo for OledLogoAlgo{
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


        OledLogoAlgo{
            display : DisplayDriver::new(get_oled_pin!(i2c_rst, pins).into_push_pull_output(), i2cw),
            color: Color::new(),
        }
    }

    fn loop_fct(&mut self) {
        let col = self.color.next();
        self.display.display.clear();
        self.display.draw_logo(col);
        sleep(1.s());
    }
}
