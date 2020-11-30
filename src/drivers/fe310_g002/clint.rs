use crate::arch::riscv32;
use register::{mmio::*, register_bitfields, register_structs};

register_structs! {
    #[allow(non_snake_case)]
    pub RegisterBlock {
        (0x0000 => MSIP0 : ReadWrite<u32>),
        (0x0004 => _reserved0),
        (0x4000 => MTIMECMP0LO : ReadWrite<u32>),
        (0x4004 => MTIMECMP0HI : ReadWrite<u32>),
        (0x4008 => _reserved1),
        (0xBFF8 => MTIME0LO : ReadWrite<u32>),
        (0xBFFC => MTIME0HI : ReadWrite<u32>),
        (0xC000 => _reserved2),
        (0xC004 => @END),
    }
}

pub struct CLINTDriver {
    base_address: usize,
}

impl core::ops::Deref for CLINTDriver {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

impl CLINTDriver {
    pub fn new(base_address: usize) -> Self {
        CLINTDriver { base_address }
    }

    fn ptr(&self) -> *const RegisterBlock {
        self.base_address as *const _
    }

    pub fn init(&self) -> crate::interface::driver::Result {
        Ok(())
    }

    pub fn get_mtime_lo(&self) -> u32 {
        self.MTIME0LO.get()
    }

    pub fn get_mtime_hi(&self) -> u32 {
        self.MTIME0HI.get()
    }

    pub fn get_mtime(&self) -> u64 {
        (self.MTIME0LO.get() as u64) + ((self.MTIME0HI.get() as u64) << 32)
    }

    fn get_timer_freq(&self) -> u32 {
        32768
    }

    // https://github.com/sifive/freedom-e-sdk/blob/v1_0/bsp/env/freedom-e300-hifive1/init.c#L141
    fn measure_cpu_freq(&self, n: u32) -> u32 {
        let mut start_mtime;
        let mut delta_mtime;

        let mtime_freq = self.get_timer_freq();

        let tmp = self.get_mtime_lo();
        loop {
            start_mtime = self.get_mtime_lo();

            if start_mtime != tmp {
                break;
            }
        }

        let mut start_mcycle = riscv32::get_csr_mcycle() as u32;

        loop {
            delta_mtime = self.get_mtime_lo() - start_mtime;

            if delta_mtime >= n {
                break;
            }
        }

        let mut end_mcycle = riscv32::get_csr_mcycle() as u32;

        let delta_mcycle = end_mcycle - start_mcycle;

        (delta_mcycle / delta_mtime) * mtime_freq
            + ((delta_mcycle % delta_mtime) * mtime_freq) / delta_mtime
    }

    pub fn get_cpu_freq(&self) -> u32 {
        self.measure_cpu_freq(1);

        self.measure_cpu_freq(10)
    }
}
