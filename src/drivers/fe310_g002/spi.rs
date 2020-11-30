use register::{mmio::*, register_bitfields, register_structs};

register_bitfields! {
    u32,

    /// Serial Clock Divisor Register
    SCKDIV [
        /// Divisor for serial clock. div_width bits wide
        DIV OFFSET(0) NUMBITS(12) []
    ],

    /// Serial Clock Mode Register
    SCKMODE [
        /// Serial clock phase
        PHA OFFSET(0) NUMBITS(1) [
            /// Data is sampled on the leading edge of SCK and shifted on the trailing edge of SCK
            LeadingEdgeSample = 0,
            /// Data is shifted on the leading edge of SCK and sampled on the trailing edge of SCK
            TrailingEdgeSample = 1
        ],

        /// Serial clock polarity
        POL OFFSET(1) NUMBITS(1) [
            /// Inactive state of SCK is logical 0
            InactiveSCKLogical0 = 0,
            /// Inactive state of SCK is logical 1
            InactiveSCKLogical1 = 1
        ]
    ],

    /// Chip Select ID Register
    CSID [
        /// Chip select ID. log_2(cs_width) bits wide.
        CSID OFFSET(0) NUMBITS(32) []
    ],

    /// Chip Select Default Register
    CSDEF [
        /// Chip select default value. cs_width bits wide, reset to all-1s.
        CSDEF OFFSET(0) NUMBITS(32) []
    ],

    /// Chip Select Mode Register
    CSMODE [
        /// Chip select mode
        MODE OFFSET(0) NUMBITS(2) [
            ///  Assert/deassert CS at the beginning/end of each frame
            AUTO = 0,
            /// Keep CS continuously asserted after the initial frame
            HOLD = 2,
            /// Disable hardware control of the CS pin
            OFF = 3
        ]
    ],

    /// Delay Control Register 0
    DELAY0  [
        /// CS to SCK Delay
        CSSCK OFFSET(0) NUMBITS(8) [],

        /// SCK to CS Delay
        SCKCS OFFSET(16) NUMBITS(8) []
    ],

    /// Delay Control Register 1
    DELAY1  [
        /// Minimum CS inactive time
        INTERCS OFFSET(0) NUMBITS(8) [],

        /// Maximum interframe delay
        INTERXFR OFFSET(16) NUMBITS(8) []
    ],

    /// Frame Format Register
    FMT [
        /// SPI protocol
        PROTO OFFSET(0) NUMBITS(2) [
            /// DQ0 (MOSI), DQ1 (MISO)
            SINGLE = 0,
            /// DQ0, DQ1
            DUAL = 1,
            /// DQ0, DQ1, DQ2, DQ3
            QUAD = 2
        ],

        /// SPI Endianness
        ENDIAN OFFSET(2) NUMBITS(1) [
            /// Transmit most-significant bit (MSB) first
            MSBFIRST = 0,
            /// Transmit least-significant bit (LSB) first
            LSBFIRST = 1
        ],

        /// SPI I/O direction. This is reset to 1 for flash-enabled SPI controllers, 0 otherwise.
        DIR OFFSET(3) NUMBITS(1) [
            /// Rx: For dual and quad protocols, the DQ pins are tri-stated. For the single protocol, the DQ0 pin is driven with the transmit data as normal.
            RX = 0,
            /// Tx: The receive FIFO is not populated.
            TX = 1
        ]
    ],

    /// Transmit Data Register
    TXDATA [
        /// Transmit Data
        DATA OFFSET(0) NUMBITS(8) [],

        /// FIFO Full Flag
        FULL OFFSET(31) NUMBITS(1) []
    ],

    /// Receive Data Register
    RXDATA [
        /// Received Data
        DATA OFFSET(0) NUMBITS(8) [],

        /// FIFO Empty Flag
        EMPTY OFFSET(31) NUMBITS(1) []
    ],

    /// Transmit Watermark Register
    TXMARK [
        /// Transmit watermark. The reset value is 1 for flash-enabled controllers, 0 otherwise.
        TXMARK OFFSET(0) NUMBITS(3) []
    ],

    /// Receive Watermark Register
    RXMARK [
        /// Receive watermark
        RXMARK OFFSET(0) NUMBITS(3) []
    ],

    /// SPI Interrupt Enable Register
    IE [
        /// Transmit watermark enable
        TXWM OFFSET(0) NUMBITS(1) [],

        /// Receive watermark enable
        RXWM OFFSET(1) NUMBITS(1) []
    ],

    /// SPI Watermark Interrupt Pending Register
    IP [
        /// Transmit watermark pending
        TXWM OFFSET(0) NUMBITS(1) [],

        /// Receive watermark pending
        RXWM OFFSET(1) NUMBITS(1) []
    ],

    /// SPI Flash Interface Control Register
    FCTRL [
        /// SPI Flash Mode Select
        EN OFFSET(0) NUMBITS(1) []
    ],

    /// SPI Flash Instruction Format Register
    FFMT [
        /// Enable sending of command
        CMDEN OFFSET(0) NUMBITS(1) [],

        /// Number of address bytes (0 to 4)
        ADDRLEN OFFSET(1) NUMBITS(3) [],

        /// Number of dummy cycles
        PADCNT OFFSET(4) NUMBITS(4) [],

        /// Protocol for transmitting command
        CMDPROTO OFFSET(8) NUMBITS(2) [],

        /// Protocol for transmitting address and padding
        ADDRPROTO OFFSET(10) NUMBITS(2) [],

        /// Protocol for receiving data bytes
        DATAPROTO OFFSET(12) NUMBITS(2) [],

        /// Value of command byte
        CMDCODE OFFSET(16) NUMBITS(8) [],

        /// First 8 bits to transmit during dummy cycles
        PADCODE OFFSET(24) NUMBITS(8) []
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    pub RegisterBlock {
        (0x00 => SCKDIV : ReadWrite<u32, SCKDIV::Register>),
        (0x04 => SCKMODE : ReadWrite<u32, SCKMODE::Register>),
        (0x08 => _reserved0),
        (0x10 => CSID : ReadWrite<u32, CSID::Register>),
        (0x14 => CSDEF : ReadWrite<u32, CSDEF::Register>),
        (0x18 => CSMODE : ReadWrite<u32, CSMODE::Register>),
        (0x1C => _reserved1),
        (0x28 => DELAY0 : ReadWrite<u32, DELAY0::Register>),
        (0x2C => DELAY1 : ReadWrite<u32, DELAY1::Register>),
        (0x30 => _reserved2),
        (0x40 => FMT : ReadWrite<u32, FMT::Register>),
        (0x44 => _reserved3),
        (0x48 => TXDATA : ReadWrite<u32, TXDATA::Register>),
        (0x4C => RXDATA : ReadWrite<u32, RXDATA::Register>),
        (0x50 => TXMARK : ReadWrite<u32, TXMARK::Register>),
        (0x54 => RXMARK : ReadWrite<u32, RXMARK::Register>),
        (0x58 => _reserved4),
        (0x60 => FCTRL : ReadWrite<u32, FCTRL::Register>),
        (0x64 => FFMT : ReadWrite<u32, FFMT::Register>),
        (0x68 => _reserved5),
        (0x70 => IE : ReadWrite<u32, IE::Register>),
        (0x74 => IP : ReadWrite<u32, IP::Register>),
        (0x78 => @END),
    }
}

pub struct SPIDriver {
    base_address: usize,
}

impl core::ops::Deref for SPIDriver {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

impl SPIDriver {
    pub fn new(base_address: usize) -> Self {
        SPIDriver { base_address }
    }

    fn ptr(&self) -> *const RegisterBlock {
        self.base_address as *const _
    }

    pub fn init(&self) -> crate::interface::driver::Result {
        Ok(())
    }

    pub fn set_clk_div(&mut self, div: u32) {
        self.SCKDIV.write(SCKDIV::DIV.val(div))
    }
}
