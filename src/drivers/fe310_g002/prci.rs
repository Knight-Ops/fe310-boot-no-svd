use crate::drivers::fe310_g002::{AONDriver, CLINTDriver, SPIDriver};
use register::{mmio::*, register_bitfields, register_structs};

register_bitfields! {
    u32,

    /// Ring Oscillator Configuration and Status
    HFROSCCFG [
        /// Ring Oscillator Divider Register
        HFROSCDIV OFFSET(0) NUMBITS(6) [],

        /// Ring Oscillator Trim Register
        HFROSCTRIM OFFSET(16) NUMBITS(5) [],

        /// Ring Oscillator Enable
        HFROSCEN OFFSET(30) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// Ring Oscillator Ready
        HFROSCRDY OFFSET(31) NUMBITS(1) [
            NotReady = 0,
            Ready = 1
        ]
    ],

    /// Crystal Oscillator Configuration and Status
    HFXOSCCFG [
        /// Crystal Oscillator Enable
        HFXOSCEN OFFSET(30) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// Crystal Oscillator Ready
        HFXOSCRDY OFFSET(31) NUMBITS(1) [
            NotReady = 0,
            Ready = 1
        ]
    ],

    /// PLL Configuration and Status
    PLLCFG [
        /// PLL R Value
        PLLR OFFSET(0) NUMBITS(3) [],

        /// PLL F Value
        PLLF OFFSET(4) NUMBITS(6) [],

        /// PLL Q Value
        PLLQ OFFSET(10) NUMBITS(2) [],

        /// PLL Select
        PLLSEL OFFSET(16) NUMBITS(1) [],

        /// PLL Reference Select
        PLLREFSEL OFFSET(17) NUMBITS(1) [],

        /// PLL Bypass
        PLLBYPASS OFFSET(18) NUMBITS(1) [],

        /// PLL Lock
        PLLLOCK OFFSET(31) NUMBITS(1) []
    ],

    /// PLL Final Divide Configuration
    PLLOUTDIV [
        /// PLL Final Divider Value
        PLLOUTDIV OFFSET(0) NUMBITS(6) [],

        /// PLL Final Divide By 1
        PLLOUTDIVBY1 OFFSET(8) NUMBITS(6) []
    ],

    /// Process Monitor Configuration and Status
    PROCMONCFG [
        /// Proccess Monitor Divide
        PROCMONDIVSEL OFFSET(0) NUMBITS(5) [],

        /// Process Monitor Delay Selector
        PROCMONDELAYSEL OFFSET(8) NUMBITS(5) [],

        /// Process Monitor Enable
        PROCMONEN OFFSET(16) NUMBITS(1) [],

        /// Process Monitor Select
        PROCMONSEL OFFSET(24) NUMBITS(2) []
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    pub RegisterBlock {
        (0x00 => HFROSCCFG: ReadWrite<u32, HFROSCCFG::Register>),
        (0x04 => HFXOSCCFG: ReadWrite<u32, HFXOSCCFG::Register>),
        (0x08 => PLLCFG: ReadWrite<u32, PLLCFG::Register>),
        (0x0C => PLLOUTDIV: ReadWrite<u32, PLLOUTDIV::Register>),
        (0x10 => _reserved0),
        (0xF0 => PROCMONCFG: ReadWrite<u32, PROCMONCFG::Register>),
        (0xF4 => @END),
    }
}

pub struct PRCIDriver {
    base_address: usize,
}

impl core::ops::Deref for PRCIDriver {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

impl PRCIDriver {
    pub fn new(base_address: usize) -> Self {
        PRCIDriver { base_address }
    }

    fn ptr(&self) -> *const RegisterBlock {
        self.base_address as *const _
    }

    pub fn init(&self) -> crate::interface::driver::Result {
        Ok(())
    }

    pub fn configure_hfrosc(&mut self, div: u32, trim: u32) {
        self.HFROSCCFG.modify(
            HFROSCCFG::HFROSCDIV.val(div)
                + HFROSCCFG::HFROSCTRIM.val(trim)
                + HFROSCCFG::HFROSCEN::Enabled,
        );

        loop {
            if self.HFROSCCFG.is_set(HFROSCCFG::HFROSCRDY) {
                break;
            }
        }

        self.PLLCFG.modify(PLLCFG::PLLSEL::CLEAR);
    }

    // https://github.com/sifive/freedom-e-sdk/blob/v1_0/bsp/env/freedom-e300-hifive1/init.c#L55
    #[inline(never)]
    pub fn setup_pll_for_use(
        &mut self,
        refsel: u32,
        bypass: bool,
        r: u32,
        f: u32,
        q: u32,
        spi: &mut SPIDriver,
        aon: &AONDriver,
    ) {
        // Ensure that we aren't running off the PLL before we mess with it.
        if self.PLLCFG.is_set(PLLCFG::PLLSEL) {
            // Make sure the HFROSC is running at its default setting
            self.configure_hfrosc(4, 16);
        }

        // Set PLL Source to be HFXOSC if available.
        let mut config_value = 0;

        config_value |= PLLCFG::PLLREFSEL.val(refsel).value;

        if bypass {
            // Bypass
            config_value |= PLLCFG::PLLBYPASS::SET.value;

            self.PLLCFG.set(config_value);

            // If we don't have an HFXTAL, this doesn't really matter.
            // Set our Final output divide to divide-by-1:
            self.PLLOUTDIV
                .write(PLLOUTDIV::PLLOUTDIVBY1.val(1) + PLLOUTDIV::PLLOUTDIV.val(0));
        } else {
            // In case we are executing from QSPI,
            // (which is quite likely) we need to
            // set the QSPI clock divider appropriately
            // before boosting the clock frequency.

            // Div = f_sck/2
            spi.set_clk_div(8);

            // Set DIV Settings for PLL
            // Both HFROSC and HFXOSC are modeled as ideal
            // 16MHz sources (assuming dividers are set properly for
            // HFROSC).
            // (Legal values of f_REF are 6-48MHz)

            // Set DIVR to divide-by-2 to get 8MHz frequency
            // (legal values of f_R are 6-12 MHz)

            config_value |= PLLCFG::PLLBYPASS::SET.value;
            config_value |= PLLCFG::PLLR.val(r).value;

            // Set DIVF to get 512Mhz frequncy
            // There is an implied multiply-by-2, 16Mhz.
            // So need to write 32-1
            // (legal values of f_F are 384-768 MHz)
            config_value |= PLLCFG::PLLF.val(f).value;

            // Set DIVQ to divide-by-2 to get 256 MHz frequency
            // (legal values of f_Q are 50-400Mhz)
            config_value |= PLLCFG::PLLQ.val(q).value;

            // Set our Final output divide to divide-by-1:
            self.PLLOUTDIV
                .write(PLLOUTDIV::PLLOUTDIVBY1.val(1) + PLLOUTDIV::PLLOUTDIV.val(0));

            self.PLLCFG.set(config_value);

            self.PLLCFG.modify(PLLCFG::PLLBYPASS::CLEAR);

            // Problem is above this
            // let now = clint.get_mtime_lo();
            let now = aon.get_rtc_lo();
            loop {
                // if clint.get_mtime_lo() - now >= 4 {
                //     break;
                // }
                if aon.get_rtc_lo() - now >= 4 {
                    break;
                }
            }

            loop {
                if self.PLLCFG.is_set(PLLCFG::PLLLOCK) {
                    break;
                }
            }
        }

        // Switch over to PLL Clock source
        self.PLLCFG.modify(PLLCFG::PLLSEL::SET);
    }

    // #[inline(never)]
    // pub fn setup_pll_for_use(&mut self, refsel: u32, bypass: bool, r: u32, f: u32, q:u32, spi: &mut SPIDriver, aon:&AONDriver) {
    //     self.PLLCFG.write(PLLCFG::PLLREFSEL.val(1) + PLLCFG::PLLR.val(1) + PLLCFG::PLLF.val(80/2-1) + PLLCFG::PLLQ.val(1));

    //     for _ in 0..100 {
    //         unsafe { asm!("nop"); }
    //     }

    //     while !self.PLLCFG.is_set(PLLCFG::PLLLOCK) {}
    //     self.PLLCFG.modify(PLLCFG::PLLSEL.val(1));
    // }
}
