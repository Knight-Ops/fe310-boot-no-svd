use register::{mmio::*, register_bitfields, register_structs};

register_bitfields! {
    u32,

    /// wdog Configuration
    WDOGCFG [
        /// Counter scale value
        WDOGSCALE OFFSET(0) NUMBITS(4) [],

        /// Controls whether the comparator output can set the wdogrst bit and hence cause a full reset.
        WDOGRSTEN OFFSET(8) NUMBITS(1) [],

        /// Reset counter to zero after match.
        WDOGZEROCMP OFFSET(9) NUMBITS(1) [],

        /// Enable Always - run continuously
        WDOGENALWAYS OFFSET(12) NUMBITS(1) [],

        /// Increment the watchdog counter if the processor is not asleep
        WDOGCOREAWAKE OFFSET(13) NUMBITS(1) [],

        /// Interrupt 0 Pending
        WDOGIP0 OFFSET(28) NUMBITS(1) []
    ],

    // /// Counter Register
    // WDOGCOUNT [],

    // /// Scaled value of Counter
    // WDOGS [],

    // /// Feed register
    // WDOGFEED [],

    // /// Key Register
    // WDOGKEY [],

    // /// Comparator 0
    // WDOGCMP0 [],

    /// rtc Configuration
    RTCCFG [
        /// Counter scale value
        RTCSCALE OFFSET(0) NUMBITS(4) [],

        /// Enable Always - run continuously
        RTCENALWAYS OFFSET(12) NUMBITS(1) [],

        /// Interrupt 0 Pending
        RTCIP0 OFFSET(28) NUMBITS(1) []
    ],

    // /// Low bits of Counter
    // RTCCOUNTLO [],

    // /// High bits of Counter
    // RTCCOUNTHI [],

    // /// Scaled value of Counter
    // RTCS [],

    // /// Comparator 0
    // RTCCMP0 [],

    /// Ring Oscillator Configuration and Status
    LFROSCCFG [
        /// Ring Oscillator Divider Register
        LFROSCDIV OFFSET(0) NUMBITS(6) [],

        /// Ring Oscillator Trim Register
        LFROSCTRIM OFFSET(16) NUMBITS(5) [],

        /// Ring Oscillator Enable
        LFROSCEN OFFSET(30) NUMBITS(1) [],

        /// Ring Oscillator Ready
        LFROSCRDY OFFSET(31) NUMBITS(1) []
    ],

    /// Low-Frequency Clock Mux Control and Status
    LFCLKMUX [
        /// Low Frequency Clock Source Selector
        LFEXTCLKSEL OFFSET(0) NUMBITS(1) [
            /// Use internal LF clock source
            Internal = 0,

            /// Use external LF clock source
            External = 1
        ],

        /// Setting of the aon_lfclksel pin
        LFEXTCLKMUXSTATUSU OFFSET(31) NUMBITS(1) [
            /// Use external LF clock source
            External = 0,

            /// Use clock source selected by lfextclk_sel
            SW = 1
        ]
    ],

    // /// Backup Register 0
    // BACKUP0 [],

    // /// Backup Register 1
    // BACKUP1 [],

    // /// Backup Register 2
    // BACKUP2 [],

    // /// Backup Register 3
    // BACKUP3 [],

    // /// Backup Register 4
    // BACKUP4 [],

    // /// Backup Register 5
    // BACKUP5 [],

    // /// Backup Register 6
    // BACKUP6 [],

    // /// Backup Register 7
    // BACKUP7 [],

    // /// Backup Register 8
    // BACKUP8 [],

    // /// Backup Register 9
    // BACKUP9 [],

    // /// Backup Register 10
    // BACKUP10 [],

    // /// Backup Register 11
    // BACKUP11 [],

    // /// Backup Register 12
    // BACKUP12 [],

    // /// Backup Register 13
    // BACKUP13 [],

    // /// Backup Register 14
    // BACKUP14 [],

    // /// Backup Register 15
    // BACKUP15 [],

    // /// Wakeup program instruction 0
    // PMUWAKEUPI0 [],

    // /// Wakeup program instruction 1
    // PMUWAKEUPI1 [],

    // /// Wakeup program instruction 2
    // PMUWAKEUPI2 [],

    // /// Wakeup program instruction 3
    // PMUWAKEUPI3 [],

    // /// Wakeup program instruction 4
    // PMUWAKEUPI4 [],

    // /// Wakeup program instruction 5
    // PMUWAKEUPI5 [],

    // /// Wakeup program instruction 6
    // PMUWAKEUPI6 [],

    // /// Wakeup program instruction 7
    // PMUWAKEUPI7 [],

    // /// Sleep program instruction 0
    // PMUSLEEPI0 [],

    // /// Sleep program instruction 1
    // PMUSLEEPI1 [],

    // /// Sleep program instruction 2
    // PMUSLEEPI2 [],

    // /// Sleep program instruction 3
    // PMUSLEEPI3 [],

    // /// Sleep program instruction 4
    // PMUSLEEPI4 [],

    // /// Sleep program instruction 5
    // PMUSLEEPI5 [],

    // /// Sleep program instruction 6
    // PMUSLEEPI6 [],

    // /// Sleep program instruction 7
    // PMUSLEEPI7 [],

    // /// PMU Interrupt Enables
    // PMUIE [],

    // /// PMU Wakeup Cause
    // PMUCAUSE [],

    // /// Initiate PMU Sleep Sequence
    // PMUSLEEP [],

    // /// PMU Key. Reads as 1 when PMU is unlocked
    // PMUKEY [],

    // /// Bandgap configuration
    // SIFIVEBANDGAP [],

    /// AON Block Configuration Information
    AONCFG [
        /// Bandgap feature is present
        HASBANDGAP OFFSET(0) NUMBITS(1) [],

        /// Brownout detector feature is present
        HASBOD OFFSET(1) NUMBITS(1) [],

        /// Low Frequency Ring Oscillator feature is present
        HASLFROSC OFFSET(2) NUMBITS(1) [],

        /// Low Frequency RC Oscillator feature is present
        HASLFRCOSC OFFSET(3) NUMBITS(1) [],

        /// Low Frequency Crystal Oscillator feature is present
        HASLFXOSC OFFSET(4) NUMBITS(1) [],

        /// Power-On-Reset feature is present
        HASPOR OFFSET(5) NUMBITS(1) [],

        /// Low Dropout Regulator feature is present
        HASLDO OFFSET(6) NUMBITS(1) []
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    pub RegisterBlock {
        (0x000 => WDOGCFG : ReadWrite<u32, WDOGCFG::Register>),
        (0x004 => _reserved),
        (0x008 => WDOGCOUNT : ReadWrite<u32>),
        (0x00C => _reserved1),
        (0x010 => WDOGS : ReadWrite<u32>),
        (0x014 => _reserved2),
        (0x018 => WDOGFEED : ReadWrite<u32>),
        (0x01C => WDOGKEY : ReadWrite<u32>),
        (0x020 => WDOGCMP0 : ReadWrite<u32>),
        (0x024 => _reserved3),
        (0x040 => RTCCFG : ReadWrite<u32, RTCCFG::Register>),
        (0x044 => _reserved4),
        (0x048 => RTCCOUNTLO : ReadWrite<u32>),
        (0x04C => RTCCOUNTHI : ReadWrite<u32>),
        (0x050 => RTCS : ReadWrite<u32>),
        (0x054 => _reserved5),
        (0x060 => RTCCMP0 : ReadWrite<u32>),
        (0x64 => _reserved6),
        (0x070 => LFROSCCFG : ReadWrite<u32, LFROSCCFG::Register>),
        (0x074 => _reserved7),
        (0x07C => LFCLKMUX : ReadWrite<u32, LFCLKMUX::Register>),
        (0x080 => BACKUP0 : ReadWrite<u32>),
        (0x084 => BACKUP1 : ReadWrite<u32>),
        (0x088 => BACKUP2 : ReadWrite<u32>),
        (0x08C => BACKUP3 : ReadWrite<u32>),
        (0x090 => BACKUP4 : ReadWrite<u32>),
        (0x094 => BACKUP5 : ReadWrite<u32>),
        (0x098 => BACKUP6 : ReadWrite<u32>),
        (0x09C => BACKUP7 : ReadWrite<u32>),
        (0x0A0 => BACKUP8 : ReadWrite<u32>),
        (0x0A4 => BACKUP9 : ReadWrite<u32>),
        (0x0A8 => BACKUP10 : ReadWrite<u32>),
        (0x0AC => BACKUP11 : ReadWrite<u32>),
        (0x0B0 => BACKUP12 : ReadWrite<u32>),
        (0x0B4 => BACKUP13 : ReadWrite<u32>),
        (0x0B8 => BACKUP14 : ReadWrite<u32>),
        (0x0BC => BACKUP15 : ReadWrite<u32>),
        (0x0C0 => _reserved8),
        (0x100 => PMUWAKEUPI0 : ReadWrite<u32>),
        (0x104 => PMUWAKEUPI1 : ReadWrite<u32>),
        (0x108 => PMUWAKEUPI2 : ReadWrite<u32>),
        (0x10C => PMUWAKEUPI3 : ReadWrite<u32>),
        (0x110 => PMUWAKEUPI4 : ReadWrite<u32>),
        (0x114 => PMUWAKEUPI5 : ReadWrite<u32>),
        (0x118 => PMUWAKEUPI6 : ReadWrite<u32>),
        (0x11C => PMUWAKEUPI7 : ReadWrite<u32>),
        (0x120 => PMUSLEEPI0 : ReadWrite<u32>),
        (0x124 => PMUSLEEPI1 : ReadWrite<u32>),
        (0x128 => PMUSLEEPI2 : ReadWrite<u32>),
        (0x12C => PMUSLEEPI3 : ReadWrite<u32>),
        (0x130 => PMUSLEEPI4 : ReadWrite<u32>),
        (0x134 => PMUSLEEPI5 : ReadWrite<u32>),
        (0x138 => PMUSLEEPI6 : ReadWrite<u32>),
        (0x13C => PMUSLEEPI7 : ReadWrite<u32>),
        (0x140 => PMUIE : ReadWrite<u32>),
        (0x144 => PMUCAUSE : ReadWrite<u32>),
        (0x148 => PMUSLEEP : ReadWrite<u32>),
        (0x14C => PMUKEY : ReadWrite<u32>),
        (0x150 => _reserved9),
        (0x210 => SIFIVEBANDGAP : ReadWrite<u32>),
        (0x214 => _reserved10),
        (0x300 => AONCFG : ReadWrite<u32, AONCFG::Register>),
        (0x304 => @END),
    }
}

pub struct AONDriver {
    base_address: usize,
}

impl core::ops::Deref for AONDriver {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

impl AONDriver {
    pub fn new(base_address: usize) -> Self {
        AONDriver { base_address }
    }

    fn ptr(&self) -> *const RegisterBlock {
        self.base_address as *const _
    }

    pub fn init(&self) -> crate::interface::driver::Result {
        self.RTCCFG.modify(RTCCFG::RTCENALWAYS::SET);
        Ok(())
    }

    pub fn disable_lfrosc(&mut self) {
        self.LFROSCCFG.modify(LFROSCCFG::LFROSCEN::CLEAR)
    }

    pub fn enable_lfrosc(&mut self) {
        self.LFROSCCFG.modify(LFROSCCFG::LFROSCEN::SET)
    }

    pub fn get_rtc_lo(&self) -> u32 {
        self.RTCCOUNTLO.get()
    }

    pub fn get_rtc_hi(&self) -> u32 {
        self.RTCCOUNTHI.get()
    }
}
