use core::fmt::Write;
use core::sync::atomic::{compiler_fence, Ordering};

use crate::arch::*;

mod memory_map;
mod peripherals;

use crate::drivers::fe310_g002::{
    AONDriver, CLINTDriver, GPIODriver, PRCIDriver, SPIDriver, UartDriver,
};
use peripherals::*;

use crate::arch::NullLock;
use crate::interface::sync::Mutex;

// Eventually we need to do this Singleton for safety of peripherals, but its more trouble
// than it is worth currently in our bootloader
// lazy_static! {
//     static ref PERIPHERALS : NullLock<Peripherals> = NullLock::new(Peripherals {
//         uart_0: Some(SifiveSerialDriver::new(memory_map::UART_0_BASE_ADDR))
//     });
// }

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    extern "C" {
        static __stack_start: u32;
    }

    // This sets up our stack pointer as soon as possible
    asm!("mv sp, {0}", in(reg) &__stack_start, options(nostack));

    // This fence doesn't seem to be required currently, but it is good form to make sure our stack is set up
    // prior to attempting to use it for anything
    compiler_fence(Ordering::SeqCst);

    // Get our operating HART ID, if we are 0, we want to init, otherwise loop forever
    if get_csr_hart_id() == 0 {
        crate::runtime_init::get().init();
    } else {
        asm!("wfi");
        loop {}
    }
}

// https://github.com/sifive/freedom-e-sdk/blob/v1_0/bsp/env/freedom-e300-hifive1/init.c#L55
fn use_default_clocks(prci: &mut PRCIDriver, aon: &mut AONDriver) {
    aon.disable_lfrosc();

    prci.configure_hfrosc(4, 16);
}

// https://github.com/sifive/freedom-e-sdk/blob/v1_0/bsp/env/freedom-e300-hifive1/init.c#L55
fn use_pll(
    refsel: u32,
    bypass: bool,
    r: u32,
    f: u32,
    q: u32,
    prci: &mut PRCIDriver,
    spi: &mut SPIDriver,
    aon: &AONDriver,
) {
    prci.setup_pll_for_use(refsel, bypass, r, f, q, spi, aon);
}

pub fn init() {
    let mut prci = PRCIDriver::new(memory_map::PRCI_BASE_ADDR);
    let mut aon = AONDriver::new(memory_map::AON_BASE_ADDR);
    aon.init();
    let mut qspi_0 = SPIDriver::new(memory_map::QSPI_0_BASE_ADDR);
    let mut spi_1 = SPIDriver::new(memory_map::SPI_1_BASE_ADDR);
    let mut spi_2 = SPIDriver::new(memory_map::SPI_2_BASE_ADDR);
    let mut gpio = GPIODriver::new(memory_map::GPIO_BASE_ADDR);
    let mut uart_0 = UartDriver::new(memory_map::UART_0_BASE_ADDR);
    let mut clint = CLINTDriver::new(memory_map::CLINT_BASE_ADDR);

    use_default_clocks(&mut prci, &mut aon);
    use_pll(0, false, 1, 31, 1, &mut prci, &mut qspi_0, &aon);

    uart_0.init(&mut gpio, &clint, 115200);

    write!(uart_0, "Testing format string {}", "RISCV-32 Board");
}
