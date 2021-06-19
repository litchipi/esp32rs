#![no_std]
#![no_main]
#![allow(unused_imports)]

use hal::prelude::*;
use esp32_hal as hal;
use esp32_hal::dprintln;
use core::panic::PanicInfo;

pub trait Algo{
    fn init() -> Self where Self: Sized;
    fn loop_fct(&mut self);
}

//TODO  Select based on configuration of Cargo.toml

mod blinky;
mod oled_simple;
mod config;
use config::WorkAlgo;

/// Entry point - called by xtensa_lx6_rt after initialisation
#[entry]
fn main() -> ! {
    let mut algo = WorkAlgo::init();
    loop {
        algo.loop_fct();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    dprintln!("\n\n*** {:?}", info);
    let mut error_blink = blinky::BlinkyAlgo::init();
    loop {
        error_blink.loop_fct();
    }
}
