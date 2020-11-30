use crate::memory;
use core::ops::Range;

unsafe fn bss_range() -> Range<*mut usize> {
    extern "C" {
        static mut __bss_start: usize;
        static mut __bss_end: usize;
    }

    Range {
        start: &mut __bss_start,
        end: &mut __bss_end,
    }
}

#[inline(always)]
unsafe fn zero_bss() {
    memory::zero_volatile(bss_range());
}

pub trait RuntimeInit {
    unsafe fn init(&self) -> ! {
        zero_bss();

        crate::bootloader_entry()
    }
}

struct Secret;

impl RuntimeInit for Secret {}

pub fn get() -> &'static dyn RuntimeInit {
    &Secret
}
