#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pin Interrupt Mode register"]
    pub isel: ISEL,
    #[doc = "0x04 - Pin Interrupt Enable (Rising) register"]
    pub ienr: IENR,
    #[doc = "0x08 - Set Pin Interrupt Enable (Rising) register"]
    pub sienr: SIENR,
    #[doc = "0x0c - Clear Pin Interrupt Enable (Rising) register"]
    pub cienr: CIENR,
    #[doc = "0x10 - Pin Interrupt Enable Falling Edge / Active Level register"]
    pub ienf: IENF,
    #[doc = "0x14 - Set Pin Interrupt Enable Falling Edge / Active Level register"]
    pub sienf: SIENF,
    #[doc = "0x18 - Clear Pin Interrupt Enable Falling Edge / Active Level address"]
    pub cienf: CIENF,
    #[doc = "0x1c - Pin Interrupt Rising Edge register"]
    pub rise: RISE,
    #[doc = "0x20 - Pin Interrupt Falling Edge register"]
    pub fall: FALL,
    #[doc = "0x24 - Pin Interrupt Status register"]
    pub ist: IST,
}
#[doc = "ISEL (rw) register accessor: an alias for `Reg<ISEL_SPEC>`"]
pub type ISEL = crate::Reg<isel::ISEL_SPEC>;
#[doc = "Pin Interrupt Mode register"]
pub mod isel;
#[doc = "IENR (rw) register accessor: an alias for `Reg<IENR_SPEC>`"]
pub type IENR = crate::Reg<ienr::IENR_SPEC>;
#[doc = "Pin Interrupt Enable (Rising) register"]
pub mod ienr;
#[doc = "SIENR (w) register accessor: an alias for `Reg<SIENR_SPEC>`"]
pub type SIENR = crate::Reg<sienr::SIENR_SPEC>;
#[doc = "Set Pin Interrupt Enable (Rising) register"]
pub mod sienr;
#[doc = "CIENR (w) register accessor: an alias for `Reg<CIENR_SPEC>`"]
pub type CIENR = crate::Reg<cienr::CIENR_SPEC>;
#[doc = "Clear Pin Interrupt Enable (Rising) register"]
pub mod cienr;
#[doc = "IENF (rw) register accessor: an alias for `Reg<IENF_SPEC>`"]
pub type IENF = crate::Reg<ienf::IENF_SPEC>;
#[doc = "Pin Interrupt Enable Falling Edge / Active Level register"]
pub mod ienf;
#[doc = "SIENF (w) register accessor: an alias for `Reg<SIENF_SPEC>`"]
pub type SIENF = crate::Reg<sienf::SIENF_SPEC>;
#[doc = "Set Pin Interrupt Enable Falling Edge / Active Level register"]
pub mod sienf;
#[doc = "CIENF (w) register accessor: an alias for `Reg<CIENF_SPEC>`"]
pub type CIENF = crate::Reg<cienf::CIENF_SPEC>;
#[doc = "Clear Pin Interrupt Enable Falling Edge / Active Level address"]
pub mod cienf;
#[doc = "RISE (rw) register accessor: an alias for `Reg<RISE_SPEC>`"]
pub type RISE = crate::Reg<rise::RISE_SPEC>;
#[doc = "Pin Interrupt Rising Edge register"]
pub mod rise;
#[doc = "FALL (rw) register accessor: an alias for `Reg<FALL_SPEC>`"]
pub type FALL = crate::Reg<fall::FALL_SPEC>;
#[doc = "Pin Interrupt Falling Edge register"]
pub mod fall;
#[doc = "IST (rw) register accessor: an alias for `Reg<IST_SPEC>`"]
pub type IST = crate::Reg<ist::IST_SPEC>;
#[doc = "Pin Interrupt Status register"]
pub mod ist;
