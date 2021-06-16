#![no_std]
#![no_main]
#![allow(unused_imports)]

use hal::prelude::*;
//use panic_halt as ;
use esp32_hal as hal;
use esp32_hal::dprintln;
use core::panic::PanicInfo;

//use esp32_hal::alloc::{Allocator, DRAM_ALLOCATOR};

pub const CORE_HZ: u32 = 40_000_000;

pub trait Algo{
    fn init() -> Self where Self: Sized;
    fn loop_fct(&mut self);
}

//TODO  Select based on configuration of Cargo.toml

//mod blinky;
//use crate::blinky::BlinkyAlgo as WorkAlgo;

mod oled_simple;
use crate::oled_simple::OledSimpleAlgo as WorkAlgo;

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
    loop {}
}
