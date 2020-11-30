pub mod sync;
pub use sync::*;

pub fn nop() {
    unsafe {
        asm!("nop");
    }
}

pub fn get_csr_hart_id() -> usize {
    let mut hart_id;

    unsafe {
        asm!("csrrs {0}, mhartid, zero", out(reg) hart_id, options(nostack));
    }

    hart_id
}

pub fn get_csr_mcycle() -> usize {
    let mut mcycle;

    unsafe {
        asm!("csrrs {0}, mcycle, zero", out(reg) mcycle, options(nostack));
    }

    mcycle
}
