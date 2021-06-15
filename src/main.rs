#![no_std]
#![no_main]

use hal::prelude::*;
use panic_halt as _;
use esp32_hal as hal;

pub const CORE_HZ: u32 = 40_000_000;

pub trait Algo{
    fn init() -> Self where Self: Sized;
    fn loop_fct(&mut self);
}

mod blinky;
use crate::blinky::BlinkyAlgo;


/// Entry point - called by xtensa_lx6_rt after initialisation
#[entry]
fn main() -> ! {
    let mut algo = BlinkyAlgo::init();
    loop {
        algo.loop_fct();
    }
}
