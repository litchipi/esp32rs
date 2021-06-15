#![no_std]
#![no_main]

use esp32_hal::target;
use hal::prelude::*;
use xtensa_lx::timer::delay;
//use panic_halt as _;
use esp32_hal as hal;

use core::panic::PanicInfo;

/// Entry point - called by xtensa_lx6_rt after initialisation
#[no_mangle]
fn main() -> ! {
    loop {}
}

/// Simple panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
