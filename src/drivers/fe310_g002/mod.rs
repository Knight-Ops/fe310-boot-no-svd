pub mod aon;
pub use aon::AONDriver;

pub mod prci;
pub use prci::PRCIDriver;

pub mod spi;
pub use spi::SPIDriver;

pub mod gpio;
pub use gpio::GPIODriver;

pub mod uart;
pub use uart::UartDriver;

pub mod clint;
pub use clint::CLINTDriver;
