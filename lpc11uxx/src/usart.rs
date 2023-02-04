#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_dll: [u8; 0x04],
    _reserved_1_dlm: [u8; 0x04],
    _reserved_2_fcr: [u8; 0x04],
    #[doc = "0x0c - Line Control Register. Contains controls for frame formatting and break generation."]
    pub lcr: LCR,
    #[doc = "0x10 - Modem Control Register."]
    pub mcr: MCR,
    #[doc = "0x14 - Line Status Register. Contains flags for transmit and receive status, including line errors."]
    pub lsr: LSR,
    #[doc = "0x18 - Modem Status Register."]
    pub msr: MSR,
    #[doc = "0x1c - Scratch Pad Register. Eight-bit temporary storage for software."]
    pub scr: SCR,
    #[doc = "0x20 - Auto-baud Control Register. Contains controls for the auto-baud feature."]
    pub acr: ACR,
    #[doc = "0x24 - IrDA Control Register. Enables and configures the IrDA (remote control) mode."]
    pub icr: ICR,
    #[doc = "0x28 - Fractional Divider Register. Generates a clock input for the baud rate divider."]
    pub fdr: FDR,
    #[doc = "0x2c - Oversampling Register. Controls the degree of oversampling during each bit time."]
    pub osr: OSR,
    #[doc = "0x30 - Transmit Enable Register. Turns off USART transmitter for use with software flow control."]
    pub ter: TER,
    _reserved13: [u8; 0x0c],
    #[doc = "0x40 - Half duplex enable register."]
    pub hden: HDEN,
    _reserved14: [u8; 0x04],
    #[doc = "0x48 - Smart Card Interface Control register. Enables and configures the Smart Card Interface feature."]
    pub scictrl: SCICTRL,
    #[doc = "0x4c - RS-485/EIA-485 Control. Contains controls to configure various aspects of RS-485/EIA-485 modes."]
    pub rs485ctrl: RS485CTRL,
    #[doc = "0x50 - RS-485/EIA-485 address match. Contains the address match value for RS-485/EIA-485 mode."]
    pub rs485adrmatch: RS485ADRMATCH,
    #[doc = "0x54 - RS-485/EIA-485 direction control delay."]
    pub rs485dly: RS485DLY,
    #[doc = "0x58 - Synchronous mode control register."]
    pub syncctrl: SYNCCTRL,
}
impl RegisterBlock {
    #[doc = "0x00 - Divisor Latch LSB. Least significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider. (DLAB=1)"]
    #[inline(always)]
    pub const fn dll(&self) -> &DLL {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - Transmit Holding Register. The next character to be transmitted is written here. (DLAB=0)"]
    #[inline(always)]
    pub const fn thr(&self) -> &THR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - Receiver Buffer Register. Contains the next received character to be read. (DLAB=0)"]
    #[inline(always)]
    pub const fn rbr(&self) -> &RBR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x04 - Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential USART interrupts. (DLAB=0)"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider. (DLAB=1)"]
    #[inline(always)]
    pub const fn dlm(&self) -> &DLM {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x08 - FIFO Control Register. Controls USART FIFO usage and modes."]
    #[inline(always)]
    pub const fn fcr(&self) -> &FCR {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x08 - Interrupt ID Register. Identifies which interrupt(s) are pending."]
    #[inline(always)]
    pub const fn iir(&self) -> &IIR {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
}
#[doc = "RBR (r) register accessor: an alias for `Reg<RBR_SPEC>`"]
pub type RBR = crate::Reg<rbr::RBR_SPEC>;
#[doc = "Receiver Buffer Register. Contains the next received character to be read. (DLAB=0)"]
pub mod rbr;
#[doc = "THR (w) register accessor: an alias for `Reg<THR_SPEC>`"]
pub type THR = crate::Reg<thr::THR_SPEC>;
#[doc = "Transmit Holding Register. The next character to be transmitted is written here. (DLAB=0)"]
pub mod thr;
#[doc = "DLL (rw) register accessor: an alias for `Reg<DLL_SPEC>`"]
pub type DLL = crate::Reg<dll::DLL_SPEC>;
#[doc = "Divisor Latch LSB. Least significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider. (DLAB=1)"]
pub mod dll;
#[doc = "DLM (rw) register accessor: an alias for `Reg<DLM_SPEC>`"]
pub type DLM = crate::Reg<dlm::DLM_SPEC>;
#[doc = "Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider. (DLAB=1)"]
pub mod dlm;
#[doc = "IER (rw) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential USART interrupts. (DLAB=0)"]
pub mod ier;
#[doc = "IIR (r) register accessor: an alias for `Reg<IIR_SPEC>`"]
pub type IIR = crate::Reg<iir::IIR_SPEC>;
#[doc = "Interrupt ID Register. Identifies which interrupt(s) are pending."]
pub mod iir;
#[doc = "FCR (w) register accessor: an alias for `Reg<FCR_SPEC>`"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "FIFO Control Register. Controls USART FIFO usage and modes."]
pub mod fcr;
#[doc = "LCR (rw) register accessor: an alias for `Reg<LCR_SPEC>`"]
pub type LCR = crate::Reg<lcr::LCR_SPEC>;
#[doc = "Line Control Register. Contains controls for frame formatting and break generation."]
pub mod lcr;
#[doc = "MCR (rw) register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "Modem Control Register."]
pub mod mcr;
#[doc = "LSR (r) register accessor: an alias for `Reg<LSR_SPEC>`"]
pub type LSR = crate::Reg<lsr::LSR_SPEC>;
#[doc = "Line Status Register. Contains flags for transmit and receive status, including line errors."]
pub mod lsr;
#[doc = "MSR (r) register accessor: an alias for `Reg<MSR_SPEC>`"]
pub type MSR = crate::Reg<msr::MSR_SPEC>;
#[doc = "Modem Status Register."]
pub mod msr;
#[doc = "SCR (rw) register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "Scratch Pad Register. Eight-bit temporary storage for software."]
pub mod scr;
#[doc = "ACR (rw) register accessor: an alias for `Reg<ACR_SPEC>`"]
pub type ACR = crate::Reg<acr::ACR_SPEC>;
#[doc = "Auto-baud Control Register. Contains controls for the auto-baud feature."]
pub mod acr;
#[doc = "ICR (rw) register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "IrDA Control Register. Enables and configures the IrDA (remote control) mode."]
pub mod icr;
#[doc = "FDR (rw) register accessor: an alias for `Reg<FDR_SPEC>`"]
pub type FDR = crate::Reg<fdr::FDR_SPEC>;
#[doc = "Fractional Divider Register. Generates a clock input for the baud rate divider."]
pub mod fdr;
#[doc = "OSR (rw) register accessor: an alias for `Reg<OSR_SPEC>`"]
pub type OSR = crate::Reg<osr::OSR_SPEC>;
#[doc = "Oversampling Register. Controls the degree of oversampling during each bit time."]
pub mod osr;
#[doc = "TER (rw) register accessor: an alias for `Reg<TER_SPEC>`"]
pub type TER = crate::Reg<ter::TER_SPEC>;
#[doc = "Transmit Enable Register. Turns off USART transmitter for use with software flow control."]
pub mod ter;
#[doc = "HDEN (rw) register accessor: an alias for `Reg<HDEN_SPEC>`"]
pub type HDEN = crate::Reg<hden::HDEN_SPEC>;
#[doc = "Half duplex enable register."]
pub mod hden;
#[doc = "SCICTRL (rw) register accessor: an alias for `Reg<SCICTRL_SPEC>`"]
pub type SCICTRL = crate::Reg<scictrl::SCICTRL_SPEC>;
#[doc = "Smart Card Interface Control register. Enables and configures the Smart Card Interface feature."]
pub mod scictrl;
#[doc = "RS485CTRL (rw) register accessor: an alias for `Reg<RS485CTRL_SPEC>`"]
pub type RS485CTRL = crate::Reg<rs485ctrl::RS485CTRL_SPEC>;
#[doc = "RS-485/EIA-485 Control. Contains controls to configure various aspects of RS-485/EIA-485 modes."]
pub mod rs485ctrl;
#[doc = "RS485ADRMATCH (rw) register accessor: an alias for `Reg<RS485ADRMATCH_SPEC>`"]
pub type RS485ADRMATCH = crate::Reg<rs485adrmatch::RS485ADRMATCH_SPEC>;
#[doc = "RS-485/EIA-485 address match. Contains the address match value for RS-485/EIA-485 mode."]
pub mod rs485adrmatch;
#[doc = "RS485DLY (rw) register accessor: an alias for `Reg<RS485DLY_SPEC>`"]
pub type RS485DLY = crate::Reg<rs485dly::RS485DLY_SPEC>;
#[doc = "RS-485/EIA-485 direction control delay."]
pub mod rs485dly;
#[doc = "SYNCCTRL (rw) register accessor: an alias for `Reg<SYNCCTRL_SPEC>`"]
pub type SYNCCTRL = crate::Reg<syncctrl::SYNCCTRL_SPEC>;
#[doc = "Synchronous mode control register."]
pub mod syncctrl;
