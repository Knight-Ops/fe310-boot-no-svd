#[cfg(feature = "bsp_hifive")]
pub mod riscv32;
#[cfg(feature = "bsp_hifive")]
pub use riscv32::*;
