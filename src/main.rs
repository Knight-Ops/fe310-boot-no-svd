#![feature(core_intrinsics)]
#![feature(asm)]
#![no_std]
#![no_main]

// #[macro_use]
// extern crate lazy_static;

mod arch;
mod bsp;
mod drivers;
mod interface;
mod memory;
mod panic;
mod runtime_init;

pub fn bootloader_entry() -> ! {
    bsp::init();

    loop {}
}
