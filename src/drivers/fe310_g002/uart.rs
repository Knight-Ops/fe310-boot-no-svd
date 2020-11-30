use super::{CLINTDriver, GPIODriver};
use register::{mmio::*, register_bitfields, register_structs};

use crate::arch::riscv32;

use core::fmt::Write;

register_bitfields! {
    u32,
    TXDATA [
        DATA OFFSET(0) NUMBITS(8) [],

        FULL OFFSET(31) NUMBITS(1) [
            EMPTY = 0,
            FULL = 1
        ]
    ],

    RXDATA [
        DATA OFFSET(0) NUMBITS(8) [],

        EMPTY OFFSET(31) NUMBITS(1) [
            EMPTY = 0,
            FULL = 1
        ]
    ],

    TXCTRL [
        TXEN OFFSET(0) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        NSTOP OFFSET(1) NUMBITS(1) [
            One = 0,
            Two = 1
        ],

        TXCNT OFFSET(16) NUMBITS(3) []
    ],

    RXCTRL [
        RXEN OFFSET(0) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        RXCNT OFFSET(16) NUMBITS(3) []
    ],

    IE [
        TXWM OFFSET(0) NUMBITS(1) [],

        RXWM OFFSET(1) NUMBITS(1) []
    ],

    IP [
        TXWM OFFSET(0) NUMBITS(1) [],

        RXWM OFFSET(1) NUMBITS(1) []
    ],

    DIV [
        DIV OFFSET(0) NUMBITS(16) []
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    pub RegisterBlock {
        (0x00 => TXDATA: ReadWrite<u32, TXDATA::Register>),
        (0x04 => RXDATA: ReadOnly<u32, RXDATA::Register>),
        (0x08 => TXCTRL: ReadWrite<u32, TXCTRL::Register>),
        (0x0C => RXCTRL: ReadWrite<u32, RXCTRL::Register>),
        (0x10 => IE    : ReadWrite<u32, IE::Register>),
        (0x14 => IP    : ReadWrite<u32, IP::Register>),
        (0x18 => DIV   : ReadWrite<u32, DIV::Register>),
        (0x1C => @END),
    }
}

pub struct UartDriver {
    base_address: usize,
    chars_written: usize,
}

impl core::ops::Deref for UartDriver {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

impl UartDriver {
    pub fn new(base_address: usize) -> Self {
        UartDriver {
            base_address,
            chars_written: 0,
        }
    }

    fn ptr(&self) -> *const RegisterBlock {
        self.base_address as *const _
    }

    pub fn init(
        &self,
        gpio: &mut GPIODriver,
        clint: &CLINTDriver,
        baud: u32,
    ) -> crate::interface::driver::Result {
        gpio.enable_uart_0();
        self.DIV.set(clint.get_cpu_freq() / baud - 1);
        self.TXCTRL.modify(TXCTRL::TXEN::Enabled);

        Ok(())
    }

    pub fn write_char(&mut self, c: char) {
        loop {
            if self.TXDATA.matches_all(TXDATA::FULL::EMPTY) {
                break;
            }

            riscv32::nop();
        }

        self.TXDATA.write(TXDATA::DATA.val(c as u32))
    }
}

impl core::fmt::Write for UartDriver {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            if c == '\n' {
                self.write_char('\r');
            }

            self.write_char('\n');
        }

        self.chars_written += s.len();

        Ok(())
    }
}
