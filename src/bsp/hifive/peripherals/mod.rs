use core::mem::replace;

use crate::drivers::fe310_g002::UartDriver;

pub struct Peripherals {
    pub uart_0: Option<UartDriver>,
}

impl Peripherals {
    pub fn take_uart_0(&mut self) -> UartDriver {
        let p = replace(&mut self.uart_0, None);
        p.unwrap()
    }
}
