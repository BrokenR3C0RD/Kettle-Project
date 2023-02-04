#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x20 - Byte pin registers port 0; pins PIO0_0 to PIO0_31"]
    pub b0: [B0; 32],
    #[doc = "0x20..0x40 - Byte pin registers port 1"]
    pub b1: [B1; 32],
    _reserved2: [u8; 0x0fc0],
    #[doc = "0x1000..0x1080 - Word pin registers port 0"]
    pub w_0: [W_0; 32],
    #[doc = "0x1080..0x1100 - Word pin registers port 1"]
    pub w_1: [W_1; 32],
    _reserved4: [u8; 0x0f00],
    #[doc = "0x2000..0x2008 - Direction registers port 0/1"]
    pub dir: [DIR; 2],
    _reserved5: [u8; 0x78],
    #[doc = "0x2080..0x2088 - Mask register port 0/1"]
    pub mask: [MASK; 2],
    _reserved6: [u8; 0x78],
    #[doc = "0x2100..0x2108 - Portpin register port 0"]
    pub pin: [PIN; 2],
    _reserved7: [u8; 0x78],
    #[doc = "0x2180..0x2188 - Masked port register port 0/1"]
    pub mpin: [MPIN; 2],
    _reserved8: [u8; 0x78],
    #[doc = "0x2200..0x2208 - Write: Set register for port 0/1 Read: output bits for port 0/1"]
    pub set: [SET; 2],
    _reserved9: [u8; 0x78],
    #[doc = "0x2280..0x2288 - Clear port 0/1"]
    pub clr: [CLR; 2],
    _reserved10: [u8; 0x78],
    #[doc = "0x2300..0x2308 - Toggle port 0/1"]
    pub not: [NOT; 2],
}
impl RegisterBlock {
    #[doc = "0x20 - Byte pin registers port 1"]
    #[inline(always)]
    pub fn b132(&self) -> &B1 {
        &self.b1[0]
    }
    #[doc = "0x21 - Byte pin registers port 1"]
    #[inline(always)]
    pub fn b133(&self) -> &B1 {
        &self.b1[1]
    }
    #[doc = "0x22 - Byte pin registers port 1"]
    #[inline(always)]
    pub fn b134(&self) -> &B1 {
        &self.b1[2]
    }
    #[doc = "0x23 - Byte pin registers port 1"]
    #[inline(always)]
    pub fn b135(&self) -> &B1 {
        &self.b1[3]
    }
    #[doc = "0x24 - Byte pin registers port 1"]
    #[inline(always)]
    pub fn b136(&self) -> &B1 {
        &self.b1[4]
    }
    #[doc = "0x25 - Byte pin registers port 1"]
    #[inline(always)]
    pub fn b137(&self) -> &B1 {
        &self.b1[5]
    }
    #[doc = "0x26 - Byte pin registers port 1"]
    #[inline(always)]
    pub fn b138(&self) -> &B1 {
        &self.b1[6]
    }
    #[doc = "0x27 - Byte pin registers port 1"]
    #[inline(always)]
    pub fn b139(&self) -> &B1 {
        &self.b1[7]
    }
    #[doc = "0x28 - Byte pin registers port 1"]
    #[inline(always)]
    pub fn b140(&self) -> &B1 {
        &self.b1[8]
    }
    #[doc = "0x29 - Byte pin registers port 1"]
    #[inline(always)]
    pub fn b141(&self) -> &B1 {
        &self.b1[9]
    }
    #[doc = "0x2a - Byte pin registers port 1"]
    #[inline(always)]
    pub fn b142(&self) -> &B1 {
        &self.b1[10]
    }
    #[doc = "0x2b - Byte pin registers port 1"]
    #[inline(always)]
    pub fn b143(&self) -> &B1 {
        &self.b1[11]
    }
    #[doc = "0x2c - Byte pin registers port 1"]
    #[inline(always)]
    pub fn b144(&self) -> &B1 {
        &self.b1[12]
    }
    #[doc = "0x2d - Byte pin registers port 1"]
    #[inline(always)]
    pub fn b145(&self) -> &B1 {
        &self.b1[13]
    }
    #[doc = "0x2e - Byte pin registers port 1"]
    #[inline(always)]
    pub fn b146(&self) -> &B1 {
        &self.b1[14]
    }
    #[doc = "0x2f - Byte pin registers port 1"]
    #[inline(always)]
    pub fn b147(&self) -> &B1 {
        &self.b1[15]
    }
    #[doc = "0x30 - Byte pin registers port 1"]
    #[inline(always)]
    pub fn b148(&self) -> &B1 {
        &self.b1[16]
    }
    #[doc = "0x31 - Byte pin registers port 1"]
    #[inline(always)]
    pub fn b149(&self) -> &B1 {
        &self.b1[17]
    }
    #[doc = "0x32 - Byte pin registers port 1"]
    #[inline(always)]
    pub fn b150(&self) -> &B1 {
        &self.b1[18]
    }
    #[doc = "0x33 - Byte pin registers port 1"]
    #[inline(always)]
    pub fn b151(&self) -> &B1 {
        &self.b1[19]
    }
    #[doc = "0x34 - Byte pin registers port 1"]
    #[inline(always)]
    pub fn b152(&self) -> &B1 {
        &self.b1[20]
    }
    #[doc = "0x35 - Byte pin registers port 1"]
    #[inline(always)]
    pub fn b153(&self) -> &B1 {
        &self.b1[21]
    }
    #[doc = "0x36 - Byte pin registers port 1"]
    #[inline(always)]
    pub fn b154(&self) -> &B1 {
        &self.b1[22]
    }
    #[doc = "0x37 - Byte pin registers port 1"]
    #[inline(always)]
    pub fn b155(&self) -> &B1 {
        &self.b1[23]
    }
    #[doc = "0x38 - Byte pin registers port 1"]
    #[inline(always)]
    pub fn b156(&self) -> &B1 {
        &self.b1[24]
    }
    #[doc = "0x39 - Byte pin registers port 1"]
    #[inline(always)]
    pub fn b157(&self) -> &B1 {
        &self.b1[25]
    }
    #[doc = "0x3a - Byte pin registers port 1"]
    #[inline(always)]
    pub fn b158(&self) -> &B1 {
        &self.b1[26]
    }
    #[doc = "0x3b - Byte pin registers port 1"]
    #[inline(always)]
    pub fn b159(&self) -> &B1 {
        &self.b1[27]
    }
    #[doc = "0x3c - Byte pin registers port 1"]
    #[inline(always)]
    pub fn b160(&self) -> &B1 {
        &self.b1[28]
    }
    #[doc = "0x3d - Byte pin registers port 1"]
    #[inline(always)]
    pub fn b161(&self) -> &B1 {
        &self.b1[29]
    }
    #[doc = "0x3e - Byte pin registers port 1"]
    #[inline(always)]
    pub fn b162(&self) -> &B1 {
        &self.b1[30]
    }
    #[doc = "0x3f - Byte pin registers port 1"]
    #[inline(always)]
    pub fn b163(&self) -> &B1 {
        &self.b1[31]
    }
    #[doc = "0x1080 - Word pin registers port 1"]
    #[inline(always)]
    pub fn w_132(&self) -> &W_1 {
        &self.w_1[0]
    }
    #[doc = "0x1084 - Word pin registers port 1"]
    #[inline(always)]
    pub fn w_133(&self) -> &W_1 {
        &self.w_1[1]
    }
    #[doc = "0x1088 - Word pin registers port 1"]
    #[inline(always)]
    pub fn w_134(&self) -> &W_1 {
        &self.w_1[2]
    }
    #[doc = "0x108c - Word pin registers port 1"]
    #[inline(always)]
    pub fn w_135(&self) -> &W_1 {
        &self.w_1[3]
    }
    #[doc = "0x1090 - Word pin registers port 1"]
    #[inline(always)]
    pub fn w_136(&self) -> &W_1 {
        &self.w_1[4]
    }
    #[doc = "0x1094 - Word pin registers port 1"]
    #[inline(always)]
    pub fn w_137(&self) -> &W_1 {
        &self.w_1[5]
    }
    #[doc = "0x1098 - Word pin registers port 1"]
    #[inline(always)]
    pub fn w_138(&self) -> &W_1 {
        &self.w_1[6]
    }
    #[doc = "0x109c - Word pin registers port 1"]
    #[inline(always)]
    pub fn w_139(&self) -> &W_1 {
        &self.w_1[7]
    }
    #[doc = "0x10a0 - Word pin registers port 1"]
    #[inline(always)]
    pub fn w_140(&self) -> &W_1 {
        &self.w_1[8]
    }
    #[doc = "0x10a4 - Word pin registers port 1"]
    #[inline(always)]
    pub fn w_141(&self) -> &W_1 {
        &self.w_1[9]
    }
    #[doc = "0x10a8 - Word pin registers port 1"]
    #[inline(always)]
    pub fn w_142(&self) -> &W_1 {
        &self.w_1[10]
    }
    #[doc = "0x10ac - Word pin registers port 1"]
    #[inline(always)]
    pub fn w_143(&self) -> &W_1 {
        &self.w_1[11]
    }
    #[doc = "0x10b0 - Word pin registers port 1"]
    #[inline(always)]
    pub fn w_144(&self) -> &W_1 {
        &self.w_1[12]
    }
    #[doc = "0x10b4 - Word pin registers port 1"]
    #[inline(always)]
    pub fn w_145(&self) -> &W_1 {
        &self.w_1[13]
    }
    #[doc = "0x10b8 - Word pin registers port 1"]
    #[inline(always)]
    pub fn w_146(&self) -> &W_1 {
        &self.w_1[14]
    }
    #[doc = "0x10bc - Word pin registers port 1"]
    #[inline(always)]
    pub fn w_147(&self) -> &W_1 {
        &self.w_1[15]
    }
    #[doc = "0x10c0 - Word pin registers port 1"]
    #[inline(always)]
    pub fn w_148(&self) -> &W_1 {
        &self.w_1[16]
    }
    #[doc = "0x10c4 - Word pin registers port 1"]
    #[inline(always)]
    pub fn w_149(&self) -> &W_1 {
        &self.w_1[17]
    }
    #[doc = "0x10c8 - Word pin registers port 1"]
    #[inline(always)]
    pub fn w_150(&self) -> &W_1 {
        &self.w_1[18]
    }
    #[doc = "0x10cc - Word pin registers port 1"]
    #[inline(always)]
    pub fn w_151(&self) -> &W_1 {
        &self.w_1[19]
    }
    #[doc = "0x10d0 - Word pin registers port 1"]
    #[inline(always)]
    pub fn w_152(&self) -> &W_1 {
        &self.w_1[20]
    }
    #[doc = "0x10d4 - Word pin registers port 1"]
    #[inline(always)]
    pub fn w_153(&self) -> &W_1 {
        &self.w_1[21]
    }
    #[doc = "0x10d8 - Word pin registers port 1"]
    #[inline(always)]
    pub fn w_154(&self) -> &W_1 {
        &self.w_1[22]
    }
    #[doc = "0x10dc - Word pin registers port 1"]
    #[inline(always)]
    pub fn w_155(&self) -> &W_1 {
        &self.w_1[23]
    }
    #[doc = "0x10e0 - Word pin registers port 1"]
    #[inline(always)]
    pub fn w_156(&self) -> &W_1 {
        &self.w_1[24]
    }
    #[doc = "0x10e4 - Word pin registers port 1"]
    #[inline(always)]
    pub fn w_157(&self) -> &W_1 {
        &self.w_1[25]
    }
    #[doc = "0x10e8 - Word pin registers port 1"]
    #[inline(always)]
    pub fn w_158(&self) -> &W_1 {
        &self.w_1[26]
    }
    #[doc = "0x10ec - Word pin registers port 1"]
    #[inline(always)]
    pub fn w_159(&self) -> &W_1 {
        &self.w_1[27]
    }
    #[doc = "0x10f0 - Word pin registers port 1"]
    #[inline(always)]
    pub fn w_160(&self) -> &W_1 {
        &self.w_1[28]
    }
    #[doc = "0x10f4 - Word pin registers port 1"]
    #[inline(always)]
    pub fn w_161(&self) -> &W_1 {
        &self.w_1[29]
    }
    #[doc = "0x10f8 - Word pin registers port 1"]
    #[inline(always)]
    pub fn w_162(&self) -> &W_1 {
        &self.w_1[30]
    }
    #[doc = "0x10fc - Word pin registers port 1"]
    #[inline(always)]
    pub fn w_163(&self) -> &W_1 {
        &self.w_1[31]
    }
}
#[doc = "B0 (rw) register accessor: an alias for `Reg<B0_SPEC>`"]
pub type B0 = crate::Reg<b0::B0_SPEC>;
#[doc = "Byte pin registers port 0; pins PIO0_0 to PIO0_31"]
pub mod b0;
#[doc = "B1 (rw) register accessor: an alias for `Reg<B1_SPEC>`"]
pub type B1 = crate::Reg<b1::B1_SPEC>;
#[doc = "Byte pin registers port 1"]
pub mod b1;
#[doc = "W_0 (rw) register accessor: an alias for `Reg<W_0_SPEC>`"]
pub type W_0 = crate::Reg<w_0::W_0_SPEC>;
#[doc = "Word pin registers port 0"]
pub mod w_0;
#[doc = "W_1 (rw) register accessor: an alias for `Reg<W_1_SPEC>`"]
pub type W_1 = crate::Reg<w_1::W_1_SPEC>;
#[doc = "Word pin registers port 1"]
pub mod w_1;
#[doc = "DIR (rw) register accessor: an alias for `Reg<DIR_SPEC>`"]
pub type DIR = crate::Reg<dir::DIR_SPEC>;
#[doc = "Direction registers port 0/1"]
pub mod dir;
#[doc = "MASK (rw) register accessor: an alias for `Reg<MASK_SPEC>`"]
pub type MASK = crate::Reg<mask::MASK_SPEC>;
#[doc = "Mask register port 0/1"]
pub mod mask;
#[doc = "PIN (rw) register accessor: an alias for `Reg<PIN_SPEC>`"]
pub type PIN = crate::Reg<pin::PIN_SPEC>;
#[doc = "Portpin register port 0"]
pub mod pin;
#[doc = "MPIN (rw) register accessor: an alias for `Reg<MPIN_SPEC>`"]
pub type MPIN = crate::Reg<mpin::MPIN_SPEC>;
#[doc = "Masked port register port 0/1"]
pub mod mpin;
#[doc = "SET (rw) register accessor: an alias for `Reg<SET_SPEC>`"]
pub type SET = crate::Reg<set::SET_SPEC>;
#[doc = "Write: Set register for port 0/1 Read: output bits for port 0/1"]
pub mod set;
#[doc = "CLR (w) register accessor: an alias for `Reg<CLR_SPEC>`"]
pub type CLR = crate::Reg<clr::CLR_SPEC>;
#[doc = "Clear port 0/1"]
pub mod clr;
#[doc = "NOT (w) register accessor: an alias for `Reg<NOT_SPEC>`"]
pub type NOT = crate::Reg<not::NOT_SPEC>;
#[doc = "Toggle port 0/1"]
pub mod not;
