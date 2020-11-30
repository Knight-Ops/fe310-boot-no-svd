use register::{mmio::*, register_bitfields, register_structs};

register_bitfields! {
    u32,

    IOF_EN [
        GPIO_16 OFFSET(16) NUMBITS(1) [],

        GPIO_17 OFFSET(17) NUMBITS(1) []
    ],

    IOF_SEL [
        GPIO_16 OFFSET(16) NUMBITS(1) [
            UART_0_RX = 0
        ],

        GPIO_17 OFFSET(17) NUMBITS(1) [
            UART_0_TX = 0
        ]

    ]
}

register_structs! {
    // #[allow(non_snake_case)]
    // pub RegisterBlock {
    //     (0x00 => INPUT_VAL : ReadWrite<u32, INPUT_VAL::Register>),
    //     (0x04 => INPUT_EN : ReadWrite<u32, INPUT_EN::Register>),
    //     (0x08 => OUTPUT_EN : ReadWrite<u32, OUTPUT_EN::Register>),
    //     (0x0C => OUTPUT_VAL : ReadWrite<u32, OUTPUT_VAL::Register>),
    //     (0x10 => PUE : ReadWrite<u32, PUE::Register>),
    //     (0x14 => DS : ReadWrite<u32, DS::Register>),
    //     (0x18 => RISE_IE : ReadWrite<u32, RISE_IE::Register>),
    //     (0x1C => RISE_IP : ReadWrite<u32, RISE_IP::Register>),
    //     (0x20 => FALL_IE : ReadWrite<u32, FALL_IE::Register>),
    //     (0x24 => FALL_IP : ReadWrite<u32, FALL_IP::Register>),
    //     (0x28 => HIGH_IE : ReadWrite<u32, HIGH_IE::Register>),
    //     (0x2C => HIGH_IP : ReadWrite<u32, HIGH_IP::Register>),
    //     (0x30 => LOW_IE : ReadWrite<u32, LOW_IE::Register>),
    //     (0x34 => LOW_IP : ReadWrite<u32, LOW_IP::Register>),
    //     (0x38 => IOF_EN : ReadWrite<u32, IOF_EN::Register>),
    //     (0x3C => IOF_SEL : ReadWrite<u32, IOF_SEL::Register>),
    //     (0x40 => OUT_XOR : ReadWrite<u32, OUT_XOR::Register>),
    //     (0x44 => @END),
    // }
    #[allow(non_snake_case)]
    pub RegisterBlock {
        (0x00 => INPUT_VAL : ReadWrite<u32>),
        (0x04 => INPUT_EN : ReadWrite<u32>),
        (0x08 => OUTPUT_EN : ReadWrite<u32>),
        (0x0C => OUTPUT_VAL : ReadWrite<u32>),
        (0x10 => PUE : ReadWrite<u32>),
        (0x14 => DS : ReadWrite<u32>),
        (0x18 => RISE_IE : ReadWrite<u32>),
        (0x1C => RISE_IP : ReadWrite<u32>),
        (0x20 => FALL_IE : ReadWrite<u32>),
        (0x24 => FALL_IP : ReadWrite<u32>),
        (0x28 => HIGH_IE : ReadWrite<u32>),
        (0x2C => HIGH_IP : ReadWrite<u32>),
        (0x30 => LOW_IE : ReadWrite<u32>),
        (0x34 => LOW_IP : ReadWrite<u32>),
        (0x38 => IOF_EN : ReadWrite<u32, IOF_EN::Register>),
        (0x3C => IOF_SEL : ReadWrite<u32, IOF_SEL::Register>),
        (0x40 => OUT_XOR : ReadWrite<u32>),
        (0x44 => @END),
    }
}

pub struct GPIODriver {
    base_address: usize,
}

impl core::ops::Deref for GPIODriver {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

impl GPIODriver {
    pub fn new(base_address: usize) -> Self {
        GPIODriver { base_address }
    }

    fn ptr(&self) -> *const RegisterBlock {
        self.base_address as *const _
    }

    pub fn init(&self) -> crate::interface::driver::Result {
        Ok(())
    }

    pub fn enable_uart_0(&mut self) {
        self.IOF_SEL
            .modify(IOF_SEL::GPIO_16::UART_0_RX + IOF_SEL::GPIO_17::UART_0_TX);
        self.IOF_EN
            .modify(IOF_EN::GPIO_16::SET + IOF_EN::GPIO_17::SET);
    }
}
