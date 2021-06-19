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

mod blinky;
mod oled_simple;
mod oled_logo;

mod config;
use config::WorkAlgo;

/// Entry point - called by xtensa_lx6_rt after initialisation
//TODO      Generate main function based on what algo to run -> build.rs
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
    loop {}
}
