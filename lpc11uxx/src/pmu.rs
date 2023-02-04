#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power control register"]
    pub pcon: PCON,
    #[doc = "0x04..0x14 - General purpose register 0"]
    pub gpreg: [GPREG; 4],
    #[doc = "0x14 - General purpose register 4"]
    pub gpreg4: GPREG4,
}
#[doc = "PCON (rw) register accessor: an alias for `Reg<PCON_SPEC>`"]
pub type PCON = crate::Reg<pcon::PCON_SPEC>;
#[doc = "Power control register"]
pub mod pcon;
#[doc = "GPREG (rw) register accessor: an alias for `Reg<GPREG_SPEC>`"]
pub type GPREG = crate::Reg<gpreg::GPREG_SPEC>;
#[doc = "General purpose register 0"]
pub mod gpreg;
#[doc = "GPREG4 (rw) register accessor: an alias for `Reg<GPREG4_SPEC>`"]
pub type GPREG4 = crate::Reg<gpreg4::GPREG4_SPEC>;
#[doc = "General purpose register 4"]
pub mod gpreg4;
