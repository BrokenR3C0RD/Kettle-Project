#[doc = "Register `PIN%s` reader"]
pub struct R(crate::R<PIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIN%s` writer"]
pub struct W(crate::W<PIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIN_SPEC>;
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
impl From<crate::W<PIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PORT0` reader - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT0_R = crate::BitReader<bool>;
#[doc = "Field `PORT0` writer - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, bool, O>;
#[doc = "Field `PORT1` reader - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT1_R = crate::BitReader<bool>;
#[doc = "Field `PORT1` writer - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, bool, O>;
#[doc = "Field `PORT2` reader - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT2_R = crate::BitReader<bool>;
#[doc = "Field `PORT2` writer - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, bool, O>;
#[doc = "Field `PORT3` reader - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT3_R = crate::BitReader<bool>;
#[doc = "Field `PORT3` writer - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, bool, O>;
#[doc = "Field `PORT4` reader - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT4_R = crate::BitReader<bool>;
#[doc = "Field `PORT4` writer - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, bool, O>;
#[doc = "Field `PORT5` reader - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT5_R = crate::BitReader<bool>;
#[doc = "Field `PORT5` writer - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, bool, O>;
#[doc = "Field `PORT6` reader - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT6_R = crate::BitReader<bool>;
#[doc = "Field `PORT6` writer - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, bool, O>;
#[doc = "Field `PORT7` reader - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT7_R = crate::BitReader<bool>;
#[doc = "Field `PORT7` writer - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, bool, O>;
#[doc = "Field `PORT8` reader - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT8_R = crate::BitReader<bool>;
#[doc = "Field `PORT8` writer - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, bool, O>;
#[doc = "Field `PORT9` reader - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT9_R = crate::BitReader<bool>;
#[doc = "Field `PORT9` writer - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT9_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, bool, O>;
#[doc = "Field `PORT10` reader - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT10_R = crate::BitReader<bool>;
#[doc = "Field `PORT10` writer - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, bool, O>;
#[doc = "Field `PORT11` reader - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT11_R = crate::BitReader<bool>;
#[doc = "Field `PORT11` writer - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, bool, O>;
#[doc = "Field `PORT12` reader - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT12_R = crate::BitReader<bool>;
#[doc = "Field `PORT12` writer - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT12_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, bool, O>;
#[doc = "Field `PORT13` reader - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT13_R = crate::BitReader<bool>;
#[doc = "Field `PORT13` writer - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, bool, O>;
#[doc = "Field `PORT14` reader - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT14_R = crate::BitReader<bool>;
#[doc = "Field `PORT14` writer - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, bool, O>;
#[doc = "Field `PORT15` reader - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT15_R = crate::BitReader<bool>;
#[doc = "Field `PORT15` writer - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, bool, O>;
#[doc = "Field `PORT16` reader - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT16_R = crate::BitReader<bool>;
#[doc = "Field `PORT16` writer - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT16_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, bool, O>;
#[doc = "Field `PORT17` reader - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT17_R = crate::BitReader<bool>;
#[doc = "Field `PORT17` writer - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT17_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, bool, O>;
#[doc = "Field `PORT18` reader - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT18_R = crate::BitReader<bool>;
#[doc = "Field `PORT18` writer - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT18_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, bool, O>;
#[doc = "Field `PORT19` reader - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT19_R = crate::BitReader<bool>;
#[doc = "Field `PORT19` writer - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT19_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, bool, O>;
#[doc = "Field `PORT20` reader - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT20_R = crate::BitReader<bool>;
#[doc = "Field `PORT20` writer - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT20_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, bool, O>;
#[doc = "Field `PORT21` reader - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT21_R = crate::BitReader<bool>;
#[doc = "Field `PORT21` writer - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT21_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, bool, O>;
#[doc = "Field `PORT22` reader - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT22_R = crate::BitReader<bool>;
#[doc = "Field `PORT22` writer - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT22_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, bool, O>;
#[doc = "Field `PORT23` reader - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT23_R = crate::BitReader<bool>;
#[doc = "Field `PORT23` writer - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT23_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, bool, O>;
#[doc = "Field `PORT24` reader - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT24_R = crate::BitReader<bool>;
#[doc = "Field `PORT24` writer - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT24_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, bool, O>;
#[doc = "Field `PORT25` reader - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT25_R = crate::BitReader<bool>;
#[doc = "Field `PORT25` writer - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT25_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, bool, O>;
#[doc = "Field `PORT26` reader - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT26_R = crate::BitReader<bool>;
#[doc = "Field `PORT26` writer - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT26_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, bool, O>;
#[doc = "Field `PORT27` reader - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT27_R = crate::BitReader<bool>;
#[doc = "Field `PORT27` writer - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT27_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, bool, O>;
#[doc = "Field `PORT28` reader - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT28_R = crate::BitReader<bool>;
#[doc = "Field `PORT28` writer - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT28_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, bool, O>;
#[doc = "Field `PORT29` reader - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT29_R = crate::BitReader<bool>;
#[doc = "Field `PORT29` writer - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT29_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, bool, O>;
#[doc = "Field `PORT30` reader - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT30_R = crate::BitReader<bool>;
#[doc = "Field `PORT30` writer - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT30_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, bool, O>;
#[doc = "Field `PORT31` reader - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT31_R = crate::BitReader<bool>;
#[doc = "Field `PORT31` writer - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PORT31_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port0(&self) -> PORT0_R {
        PORT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port1(&self) -> PORT1_R {
        PORT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port2(&self) -> PORT2_R {
        PORT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port3(&self) -> PORT3_R {
        PORT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port4(&self) -> PORT4_R {
        PORT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port5(&self) -> PORT5_R {
        PORT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port6(&self) -> PORT6_R {
        PORT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port7(&self) -> PORT7_R {
        PORT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port8(&self) -> PORT8_R {
        PORT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port9(&self) -> PORT9_R {
        PORT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port10(&self) -> PORT10_R {
        PORT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port11(&self) -> PORT11_R {
        PORT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port12(&self) -> PORT12_R {
        PORT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port13(&self) -> PORT13_R {
        PORT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port14(&self) -> PORT14_R {
        PORT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port15(&self) -> PORT15_R {
        PORT15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port16(&self) -> PORT16_R {
        PORT16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port17(&self) -> PORT17_R {
        PORT17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port18(&self) -> PORT18_R {
        PORT18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port19(&self) -> PORT19_R {
        PORT19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port20(&self) -> PORT20_R {
        PORT20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port21(&self) -> PORT21_R {
        PORT21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port22(&self) -> PORT22_R {
        PORT22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port23(&self) -> PORT23_R {
        PORT23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port24(&self) -> PORT24_R {
        PORT24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port25(&self) -> PORT25_R {
        PORT25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port26(&self) -> PORT26_R {
        PORT26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port27(&self) -> PORT27_R {
        PORT27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port28(&self) -> PORT28_R {
        PORT28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port29(&self) -> PORT29_R {
        PORT29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port30(&self) -> PORT30_R {
        PORT30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port31(&self) -> PORT31_R {
        PORT31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn port0(&mut self) -> PORT0_W<0> {
        PORT0_W::new(self)
    }
    #[doc = "Bit 1 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn port1(&mut self) -> PORT1_W<1> {
        PORT1_W::new(self)
    }
    #[doc = "Bit 2 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn port2(&mut self) -> PORT2_W<2> {
        PORT2_W::new(self)
    }
    #[doc = "Bit 3 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn port3(&mut self) -> PORT3_W<3> {
        PORT3_W::new(self)
    }
    #[doc = "Bit 4 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn port4(&mut self) -> PORT4_W<4> {
        PORT4_W::new(self)
    }
    #[doc = "Bit 5 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn port5(&mut self) -> PORT5_W<5> {
        PORT5_W::new(self)
    }
    #[doc = "Bit 6 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn port6(&mut self) -> PORT6_W<6> {
        PORT6_W::new(self)
    }
    #[doc = "Bit 7 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn port7(&mut self) -> PORT7_W<7> {
        PORT7_W::new(self)
    }
    #[doc = "Bit 8 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn port8(&mut self) -> PORT8_W<8> {
        PORT8_W::new(self)
    }
    #[doc = "Bit 9 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn port9(&mut self) -> PORT9_W<9> {
        PORT9_W::new(self)
    }
    #[doc = "Bit 10 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn port10(&mut self) -> PORT10_W<10> {
        PORT10_W::new(self)
    }
    #[doc = "Bit 11 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn port11(&mut self) -> PORT11_W<11> {
        PORT11_W::new(self)
    }
    #[doc = "Bit 12 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn port12(&mut self) -> PORT12_W<12> {
        PORT12_W::new(self)
    }
    #[doc = "Bit 13 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn port13(&mut self) -> PORT13_W<13> {
        PORT13_W::new(self)
    }
    #[doc = "Bit 14 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn port14(&mut self) -> PORT14_W<14> {
        PORT14_W::new(self)
    }
    #[doc = "Bit 15 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn port15(&mut self) -> PORT15_W<15> {
        PORT15_W::new(self)
    }
    #[doc = "Bit 16 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn port16(&mut self) -> PORT16_W<16> {
        PORT16_W::new(self)
    }
    #[doc = "Bit 17 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn port17(&mut self) -> PORT17_W<17> {
        PORT17_W::new(self)
    }
    #[doc = "Bit 18 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn port18(&mut self) -> PORT18_W<18> {
        PORT18_W::new(self)
    }
    #[doc = "Bit 19 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn port19(&mut self) -> PORT19_W<19> {
        PORT19_W::new(self)
    }
    #[doc = "Bit 20 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn port20(&mut self) -> PORT20_W<20> {
        PORT20_W::new(self)
    }
    #[doc = "Bit 21 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn port21(&mut self) -> PORT21_W<21> {
        PORT21_W::new(self)
    }
    #[doc = "Bit 22 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn port22(&mut self) -> PORT22_W<22> {
        PORT22_W::new(self)
    }
    #[doc = "Bit 23 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn port23(&mut self) -> PORT23_W<23> {
        PORT23_W::new(self)
    }
    #[doc = "Bit 24 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn port24(&mut self) -> PORT24_W<24> {
        PORT24_W::new(self)
    }
    #[doc = "Bit 25 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn port25(&mut self) -> PORT25_W<25> {
        PORT25_W::new(self)
    }
    #[doc = "Bit 26 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn port26(&mut self) -> PORT26_W<26> {
        PORT26_W::new(self)
    }
    #[doc = "Bit 27 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn port27(&mut self) -> PORT27_W<27> {
        PORT27_W::new(self)
    }
    #[doc = "Bit 28 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn port28(&mut self) -> PORT28_W<28> {
        PORT28_W::new(self)
    }
    #[doc = "Bit 29 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn port29(&mut self) -> PORT29_W<29> {
        PORT29_W::new(self)
    }
    #[doc = "Bit 30 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn port30(&mut self) -> PORT30_W<30> {
        PORT30_W::new(self)
    }
    #[doc = "Bit 31 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn port31(&mut self) -> PORT31_W<31> {
        PORT31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Portpin register port 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pin](index.html) module"]
pub struct PIN_SPEC;
impl crate::RegisterSpec for PIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pin::R](R) reader structure"]
impl crate::Readable for PIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pin::W](W) writer structure"]
impl crate::Writable for PIN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIN%s to value 0"]
impl crate::Resettable for PIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
