#[doc = "Register `MASK%s` reader"]
pub struct R(crate::R<MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MASK%s` writer"]
pub struct W(crate::W<MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASKP0` reader - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP0_R = crate::BitReader<bool>;
#[doc = "Field `MASKP0` writer - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `MASKP1` reader - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP1_R = crate::BitReader<bool>;
#[doc = "Field `MASKP1` writer - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `MASKP2` reader - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP2_R = crate::BitReader<bool>;
#[doc = "Field `MASKP2` writer - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `MASKP3` reader - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP3_R = crate::BitReader<bool>;
#[doc = "Field `MASKP3` writer - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `MASKP4` reader - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP4_R = crate::BitReader<bool>;
#[doc = "Field `MASKP4` writer - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `MASKP5` reader - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP5_R = crate::BitReader<bool>;
#[doc = "Field `MASKP5` writer - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `MASKP6` reader - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP6_R = crate::BitReader<bool>;
#[doc = "Field `MASKP6` writer - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP6_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `MASKP7` reader - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP7_R = crate::BitReader<bool>;
#[doc = "Field `MASKP7` writer - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP7_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `MASKP8` reader - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP8_R = crate::BitReader<bool>;
#[doc = "Field `MASKP8` writer - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP8_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `MASKP9` reader - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP9_R = crate::BitReader<bool>;
#[doc = "Field `MASKP9` writer - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP9_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `MASKP10` reader - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP10_R = crate::BitReader<bool>;
#[doc = "Field `MASKP10` writer - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP10_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `MASKP11` reader - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP11_R = crate::BitReader<bool>;
#[doc = "Field `MASKP11` writer - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP11_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `MASKP12` reader - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP12_R = crate::BitReader<bool>;
#[doc = "Field `MASKP12` writer - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP12_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `MASKP13` reader - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP13_R = crate::BitReader<bool>;
#[doc = "Field `MASKP13` writer - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP13_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `MASKP14` reader - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP14_R = crate::BitReader<bool>;
#[doc = "Field `MASKP14` writer - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP14_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `MASKP15` reader - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP15_R = crate::BitReader<bool>;
#[doc = "Field `MASKP15` writer - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP15_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `MASKP16` reader - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP16_R = crate::BitReader<bool>;
#[doc = "Field `MASKP16` writer - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP16_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `MASKP17` reader - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP17_R = crate::BitReader<bool>;
#[doc = "Field `MASKP17` writer - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP17_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `MASKP18` reader - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP18_R = crate::BitReader<bool>;
#[doc = "Field `MASKP18` writer - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP18_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `MASKP19` reader - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP19_R = crate::BitReader<bool>;
#[doc = "Field `MASKP19` writer - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP19_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `MASKP20` reader - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP20_R = crate::BitReader<bool>;
#[doc = "Field `MASKP20` writer - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP20_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `MASKP21` reader - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP21_R = crate::BitReader<bool>;
#[doc = "Field `MASKP21` writer - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP21_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `MASKP22` reader - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP22_R = crate::BitReader<bool>;
#[doc = "Field `MASKP22` writer - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP22_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `MASKP23` reader - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP23_R = crate::BitReader<bool>;
#[doc = "Field `MASKP23` writer - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP23_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `MASKP24` reader - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP24_R = crate::BitReader<bool>;
#[doc = "Field `MASKP24` writer - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP24_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `MASKP25` reader - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP25_R = crate::BitReader<bool>;
#[doc = "Field `MASKP25` writer - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP25_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `MASKP26` reader - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP26_R = crate::BitReader<bool>;
#[doc = "Field `MASKP26` writer - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP26_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `MASKP27` reader - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP27_R = crate::BitReader<bool>;
#[doc = "Field `MASKP27` writer - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP27_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `MASKP28` reader - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP28_R = crate::BitReader<bool>;
#[doc = "Field `MASKP28` writer - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP28_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `MASKP29` reader - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP29_R = crate::BitReader<bool>;
#[doc = "Field `MASKP29` writer - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP29_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `MASKP30` reader - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP30_R = crate::BitReader<bool>;
#[doc = "Field `MASKP30` writer - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP30_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `MASKP31` reader - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP31_R = crate::BitReader<bool>;
#[doc = "Field `MASKP31` writer - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MASKP31_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp0(&self) -> MASKP0_R {
        MASKP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp1(&self) -> MASKP1_R {
        MASKP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp2(&self) -> MASKP2_R {
        MASKP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp3(&self) -> MASKP3_R {
        MASKP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp4(&self) -> MASKP4_R {
        MASKP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp5(&self) -> MASKP5_R {
        MASKP5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp6(&self) -> MASKP6_R {
        MASKP6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp7(&self) -> MASKP7_R {
        MASKP7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp8(&self) -> MASKP8_R {
        MASKP8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp9(&self) -> MASKP9_R {
        MASKP9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp10(&self) -> MASKP10_R {
        MASKP10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp11(&self) -> MASKP11_R {
        MASKP11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp12(&self) -> MASKP12_R {
        MASKP12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp13(&self) -> MASKP13_R {
        MASKP13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp14(&self) -> MASKP14_R {
        MASKP14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp15(&self) -> MASKP15_R {
        MASKP15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp16(&self) -> MASKP16_R {
        MASKP16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp17(&self) -> MASKP17_R {
        MASKP17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp18(&self) -> MASKP18_R {
        MASKP18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp19(&self) -> MASKP19_R {
        MASKP19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp20(&self) -> MASKP20_R {
        MASKP20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp21(&self) -> MASKP21_R {
        MASKP21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp22(&self) -> MASKP22_R {
        MASKP22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp23(&self) -> MASKP23_R {
        MASKP23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp24(&self) -> MASKP24_R {
        MASKP24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp25(&self) -> MASKP25_R {
        MASKP25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp26(&self) -> MASKP26_R {
        MASKP26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp27(&self) -> MASKP27_R {
        MASKP27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp28(&self) -> MASKP28_R {
        MASKP28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp29(&self) -> MASKP29_R {
        MASKP29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp30(&self) -> MASKP30_R {
        MASKP30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp31(&self) -> MASKP31_R {
        MASKP31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    #[must_use]
    pub fn maskp0(&mut self) -> MASKP0_W<0> {
        MASKP0_W::new(self)
    }
    #[doc = "Bit 1 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    #[must_use]
    pub fn maskp1(&mut self) -> MASKP1_W<1> {
        MASKP1_W::new(self)
    }
    #[doc = "Bit 2 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    #[must_use]
    pub fn maskp2(&mut self) -> MASKP2_W<2> {
        MASKP2_W::new(self)
    }
    #[doc = "Bit 3 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    #[must_use]
    pub fn maskp3(&mut self) -> MASKP3_W<3> {
        MASKP3_W::new(self)
    }
    #[doc = "Bit 4 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    #[must_use]
    pub fn maskp4(&mut self) -> MASKP4_W<4> {
        MASKP4_W::new(self)
    }
    #[doc = "Bit 5 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    #[must_use]
    pub fn maskp5(&mut self) -> MASKP5_W<5> {
        MASKP5_W::new(self)
    }
    #[doc = "Bit 6 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    #[must_use]
    pub fn maskp6(&mut self) -> MASKP6_W<6> {
        MASKP6_W::new(self)
    }
    #[doc = "Bit 7 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    #[must_use]
    pub fn maskp7(&mut self) -> MASKP7_W<7> {
        MASKP7_W::new(self)
    }
    #[doc = "Bit 8 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    #[must_use]
    pub fn maskp8(&mut self) -> MASKP8_W<8> {
        MASKP8_W::new(self)
    }
    #[doc = "Bit 9 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    #[must_use]
    pub fn maskp9(&mut self) -> MASKP9_W<9> {
        MASKP9_W::new(self)
    }
    #[doc = "Bit 10 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    #[must_use]
    pub fn maskp10(&mut self) -> MASKP10_W<10> {
        MASKP10_W::new(self)
    }
    #[doc = "Bit 11 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    #[must_use]
    pub fn maskp11(&mut self) -> MASKP11_W<11> {
        MASKP11_W::new(self)
    }
    #[doc = "Bit 12 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    #[must_use]
    pub fn maskp12(&mut self) -> MASKP12_W<12> {
        MASKP12_W::new(self)
    }
    #[doc = "Bit 13 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    #[must_use]
    pub fn maskp13(&mut self) -> MASKP13_W<13> {
        MASKP13_W::new(self)
    }
    #[doc = "Bit 14 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    #[must_use]
    pub fn maskp14(&mut self) -> MASKP14_W<14> {
        MASKP14_W::new(self)
    }
    #[doc = "Bit 15 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    #[must_use]
    pub fn maskp15(&mut self) -> MASKP15_W<15> {
        MASKP15_W::new(self)
    }
    #[doc = "Bit 16 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    #[must_use]
    pub fn maskp16(&mut self) -> MASKP16_W<16> {
        MASKP16_W::new(self)
    }
    #[doc = "Bit 17 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    #[must_use]
    pub fn maskp17(&mut self) -> MASKP17_W<17> {
        MASKP17_W::new(self)
    }
    #[doc = "Bit 18 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    #[must_use]
    pub fn maskp18(&mut self) -> MASKP18_W<18> {
        MASKP18_W::new(self)
    }
    #[doc = "Bit 19 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    #[must_use]
    pub fn maskp19(&mut self) -> MASKP19_W<19> {
        MASKP19_W::new(self)
    }
    #[doc = "Bit 20 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    #[must_use]
    pub fn maskp20(&mut self) -> MASKP20_W<20> {
        MASKP20_W::new(self)
    }
    #[doc = "Bit 21 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    #[must_use]
    pub fn maskp21(&mut self) -> MASKP21_W<21> {
        MASKP21_W::new(self)
    }
    #[doc = "Bit 22 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    #[must_use]
    pub fn maskp22(&mut self) -> MASKP22_W<22> {
        MASKP22_W::new(self)
    }
    #[doc = "Bit 23 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    #[must_use]
    pub fn maskp23(&mut self) -> MASKP23_W<23> {
        MASKP23_W::new(self)
    }
    #[doc = "Bit 24 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    #[must_use]
    pub fn maskp24(&mut self) -> MASKP24_W<24> {
        MASKP24_W::new(self)
    }
    #[doc = "Bit 25 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    #[must_use]
    pub fn maskp25(&mut self) -> MASKP25_W<25> {
        MASKP25_W::new(self)
    }
    #[doc = "Bit 26 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    #[must_use]
    pub fn maskp26(&mut self) -> MASKP26_W<26> {
        MASKP26_W::new(self)
    }
    #[doc = "Bit 27 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    #[must_use]
    pub fn maskp27(&mut self) -> MASKP27_W<27> {
        MASKP27_W::new(self)
    }
    #[doc = "Bit 28 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    #[must_use]
    pub fn maskp28(&mut self) -> MASKP28_W<28> {
        MASKP28_W::new(self)
    }
    #[doc = "Bit 29 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    #[must_use]
    pub fn maskp29(&mut self) -> MASKP29_W<29> {
        MASKP29_W::new(self)
    }
    #[doc = "Bit 30 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    #[must_use]
    pub fn maskp30(&mut self) -> MASKP30_W<30> {
        MASKP30_W::new(self)
    }
    #[doc = "Bit 31 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    #[must_use]
    pub fn maskp31(&mut self) -> MASKP31_W<31> {
        MASKP31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mask register port 0/1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask](index.html) module"]
pub struct MASK_SPEC;
impl crate::RegisterSpec for MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mask::R](R) reader structure"]
impl crate::Readable for MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mask::W](W) writer structure"]
impl crate::Writable for MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MASK%s to value 0"]
impl crate::Resettable for MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
