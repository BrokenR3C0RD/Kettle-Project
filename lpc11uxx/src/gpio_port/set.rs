#[doc = "Register `SET%s` reader"]
pub struct R(crate::R<SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SET%s` writer"]
pub struct W(crate::W<SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SET_SPEC>;
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
impl From<crate::W<SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SETP0` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP0_R = crate::BitReader<bool>;
#[doc = "Field `SETP0` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SET_SPEC, bool, O>;
#[doc = "Field `SETP1` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP1_R = crate::BitReader<bool>;
#[doc = "Field `SETP1` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SET_SPEC, bool, O>;
#[doc = "Field `SETP2` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP2_R = crate::BitReader<bool>;
#[doc = "Field `SETP2` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SET_SPEC, bool, O>;
#[doc = "Field `SETP3` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP3_R = crate::BitReader<bool>;
#[doc = "Field `SETP3` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SET_SPEC, bool, O>;
#[doc = "Field `SETP4` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP4_R = crate::BitReader<bool>;
#[doc = "Field `SETP4` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, SET_SPEC, bool, O>;
#[doc = "Field `SETP5` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP5_R = crate::BitReader<bool>;
#[doc = "Field `SETP5` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, SET_SPEC, bool, O>;
#[doc = "Field `SETP6` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP6_R = crate::BitReader<bool>;
#[doc = "Field `SETP6` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP6_W<'a, const O: u8> = crate::BitWriter<'a, u32, SET_SPEC, bool, O>;
#[doc = "Field `SETP7` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP7_R = crate::BitReader<bool>;
#[doc = "Field `SETP7` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP7_W<'a, const O: u8> = crate::BitWriter<'a, u32, SET_SPEC, bool, O>;
#[doc = "Field `SETP8` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP8_R = crate::BitReader<bool>;
#[doc = "Field `SETP8` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP8_W<'a, const O: u8> = crate::BitWriter<'a, u32, SET_SPEC, bool, O>;
#[doc = "Field `SETP9` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP9_R = crate::BitReader<bool>;
#[doc = "Field `SETP9` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP9_W<'a, const O: u8> = crate::BitWriter<'a, u32, SET_SPEC, bool, O>;
#[doc = "Field `SETP10` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP10_R = crate::BitReader<bool>;
#[doc = "Field `SETP10` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP10_W<'a, const O: u8> = crate::BitWriter<'a, u32, SET_SPEC, bool, O>;
#[doc = "Field `SETP11` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP11_R = crate::BitReader<bool>;
#[doc = "Field `SETP11` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP11_W<'a, const O: u8> = crate::BitWriter<'a, u32, SET_SPEC, bool, O>;
#[doc = "Field `SETP12` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP12_R = crate::BitReader<bool>;
#[doc = "Field `SETP12` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP12_W<'a, const O: u8> = crate::BitWriter<'a, u32, SET_SPEC, bool, O>;
#[doc = "Field `SETP13` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP13_R = crate::BitReader<bool>;
#[doc = "Field `SETP13` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP13_W<'a, const O: u8> = crate::BitWriter<'a, u32, SET_SPEC, bool, O>;
#[doc = "Field `SETP14` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP14_R = crate::BitReader<bool>;
#[doc = "Field `SETP14` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP14_W<'a, const O: u8> = crate::BitWriter<'a, u32, SET_SPEC, bool, O>;
#[doc = "Field `SETP15` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP15_R = crate::BitReader<bool>;
#[doc = "Field `SETP15` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP15_W<'a, const O: u8> = crate::BitWriter<'a, u32, SET_SPEC, bool, O>;
#[doc = "Field `SETP16` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP16_R = crate::BitReader<bool>;
#[doc = "Field `SETP16` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP16_W<'a, const O: u8> = crate::BitWriter<'a, u32, SET_SPEC, bool, O>;
#[doc = "Field `SETP17` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP17_R = crate::BitReader<bool>;
#[doc = "Field `SETP17` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP17_W<'a, const O: u8> = crate::BitWriter<'a, u32, SET_SPEC, bool, O>;
#[doc = "Field `SETP18` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP18_R = crate::BitReader<bool>;
#[doc = "Field `SETP18` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP18_W<'a, const O: u8> = crate::BitWriter<'a, u32, SET_SPEC, bool, O>;
#[doc = "Field `SETP19` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP19_R = crate::BitReader<bool>;
#[doc = "Field `SETP19` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP19_W<'a, const O: u8> = crate::BitWriter<'a, u32, SET_SPEC, bool, O>;
#[doc = "Field `SETP20` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP20_R = crate::BitReader<bool>;
#[doc = "Field `SETP20` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP20_W<'a, const O: u8> = crate::BitWriter<'a, u32, SET_SPEC, bool, O>;
#[doc = "Field `SETP21` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP21_R = crate::BitReader<bool>;
#[doc = "Field `SETP21` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP21_W<'a, const O: u8> = crate::BitWriter<'a, u32, SET_SPEC, bool, O>;
#[doc = "Field `SETP22` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP22_R = crate::BitReader<bool>;
#[doc = "Field `SETP22` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP22_W<'a, const O: u8> = crate::BitWriter<'a, u32, SET_SPEC, bool, O>;
#[doc = "Field `SETP23` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP23_R = crate::BitReader<bool>;
#[doc = "Field `SETP23` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP23_W<'a, const O: u8> = crate::BitWriter<'a, u32, SET_SPEC, bool, O>;
#[doc = "Field `SETP24` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP24_R = crate::BitReader<bool>;
#[doc = "Field `SETP24` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP24_W<'a, const O: u8> = crate::BitWriter<'a, u32, SET_SPEC, bool, O>;
#[doc = "Field `SETP25` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP25_R = crate::BitReader<bool>;
#[doc = "Field `SETP25` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP25_W<'a, const O: u8> = crate::BitWriter<'a, u32, SET_SPEC, bool, O>;
#[doc = "Field `SETP26` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP26_R = crate::BitReader<bool>;
#[doc = "Field `SETP26` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP26_W<'a, const O: u8> = crate::BitWriter<'a, u32, SET_SPEC, bool, O>;
#[doc = "Field `SETP27` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP27_R = crate::BitReader<bool>;
#[doc = "Field `SETP27` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP27_W<'a, const O: u8> = crate::BitWriter<'a, u32, SET_SPEC, bool, O>;
#[doc = "Field `SETP28` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP28_R = crate::BitReader<bool>;
#[doc = "Field `SETP28` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP28_W<'a, const O: u8> = crate::BitWriter<'a, u32, SET_SPEC, bool, O>;
#[doc = "Field `SETP29` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP29_R = crate::BitReader<bool>;
#[doc = "Field `SETP29` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP29_W<'a, const O: u8> = crate::BitWriter<'a, u32, SET_SPEC, bool, O>;
#[doc = "Field `SETP30` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP30_R = crate::BitReader<bool>;
#[doc = "Field `SETP30` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP30_W<'a, const O: u8> = crate::BitWriter<'a, u32, SET_SPEC, bool, O>;
#[doc = "Field `SETP31` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP31_R = crate::BitReader<bool>;
#[doc = "Field `SETP31` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SETP31_W<'a, const O: u8> = crate::BitWriter<'a, u32, SET_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp0(&self) -> SETP0_R {
        SETP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp1(&self) -> SETP1_R {
        SETP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp2(&self) -> SETP2_R {
        SETP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp3(&self) -> SETP3_R {
        SETP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp4(&self) -> SETP4_R {
        SETP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp5(&self) -> SETP5_R {
        SETP5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp6(&self) -> SETP6_R {
        SETP6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp7(&self) -> SETP7_R {
        SETP7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp8(&self) -> SETP8_R {
        SETP8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp9(&self) -> SETP9_R {
        SETP9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp10(&self) -> SETP10_R {
        SETP10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp11(&self) -> SETP11_R {
        SETP11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp12(&self) -> SETP12_R {
        SETP12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp13(&self) -> SETP13_R {
        SETP13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp14(&self) -> SETP14_R {
        SETP14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp15(&self) -> SETP15_R {
        SETP15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp16(&self) -> SETP16_R {
        SETP16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp17(&self) -> SETP17_R {
        SETP17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp18(&self) -> SETP18_R {
        SETP18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp19(&self) -> SETP19_R {
        SETP19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp20(&self) -> SETP20_R {
        SETP20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp21(&self) -> SETP21_R {
        SETP21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp22(&self) -> SETP22_R {
        SETP22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp23(&self) -> SETP23_R {
        SETP23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp24(&self) -> SETP24_R {
        SETP24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp25(&self) -> SETP25_R {
        SETP25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp26(&self) -> SETP26_R {
        SETP26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp27(&self) -> SETP27_R {
        SETP27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp28(&self) -> SETP28_R {
        SETP28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp29(&self) -> SETP29_R {
        SETP29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp30(&self) -> SETP30_R {
        SETP30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp31(&self) -> SETP31_R {
        SETP31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn setp0(&mut self) -> SETP0_W<0> {
        SETP0_W::new(self)
    }
    #[doc = "Bit 1 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn setp1(&mut self) -> SETP1_W<1> {
        SETP1_W::new(self)
    }
    #[doc = "Bit 2 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn setp2(&mut self) -> SETP2_W<2> {
        SETP2_W::new(self)
    }
    #[doc = "Bit 3 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn setp3(&mut self) -> SETP3_W<3> {
        SETP3_W::new(self)
    }
    #[doc = "Bit 4 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn setp4(&mut self) -> SETP4_W<4> {
        SETP4_W::new(self)
    }
    #[doc = "Bit 5 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn setp5(&mut self) -> SETP5_W<5> {
        SETP5_W::new(self)
    }
    #[doc = "Bit 6 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn setp6(&mut self) -> SETP6_W<6> {
        SETP6_W::new(self)
    }
    #[doc = "Bit 7 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn setp7(&mut self) -> SETP7_W<7> {
        SETP7_W::new(self)
    }
    #[doc = "Bit 8 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn setp8(&mut self) -> SETP8_W<8> {
        SETP8_W::new(self)
    }
    #[doc = "Bit 9 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn setp9(&mut self) -> SETP9_W<9> {
        SETP9_W::new(self)
    }
    #[doc = "Bit 10 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn setp10(&mut self) -> SETP10_W<10> {
        SETP10_W::new(self)
    }
    #[doc = "Bit 11 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn setp11(&mut self) -> SETP11_W<11> {
        SETP11_W::new(self)
    }
    #[doc = "Bit 12 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn setp12(&mut self) -> SETP12_W<12> {
        SETP12_W::new(self)
    }
    #[doc = "Bit 13 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn setp13(&mut self) -> SETP13_W<13> {
        SETP13_W::new(self)
    }
    #[doc = "Bit 14 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn setp14(&mut self) -> SETP14_W<14> {
        SETP14_W::new(self)
    }
    #[doc = "Bit 15 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn setp15(&mut self) -> SETP15_W<15> {
        SETP15_W::new(self)
    }
    #[doc = "Bit 16 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn setp16(&mut self) -> SETP16_W<16> {
        SETP16_W::new(self)
    }
    #[doc = "Bit 17 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn setp17(&mut self) -> SETP17_W<17> {
        SETP17_W::new(self)
    }
    #[doc = "Bit 18 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn setp18(&mut self) -> SETP18_W<18> {
        SETP18_W::new(self)
    }
    #[doc = "Bit 19 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn setp19(&mut self) -> SETP19_W<19> {
        SETP19_W::new(self)
    }
    #[doc = "Bit 20 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn setp20(&mut self) -> SETP20_W<20> {
        SETP20_W::new(self)
    }
    #[doc = "Bit 21 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn setp21(&mut self) -> SETP21_W<21> {
        SETP21_W::new(self)
    }
    #[doc = "Bit 22 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn setp22(&mut self) -> SETP22_W<22> {
        SETP22_W::new(self)
    }
    #[doc = "Bit 23 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn setp23(&mut self) -> SETP23_W<23> {
        SETP23_W::new(self)
    }
    #[doc = "Bit 24 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn setp24(&mut self) -> SETP24_W<24> {
        SETP24_W::new(self)
    }
    #[doc = "Bit 25 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn setp25(&mut self) -> SETP25_W<25> {
        SETP25_W::new(self)
    }
    #[doc = "Bit 26 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn setp26(&mut self) -> SETP26_W<26> {
        SETP26_W::new(self)
    }
    #[doc = "Bit 27 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn setp27(&mut self) -> SETP27_W<27> {
        SETP27_W::new(self)
    }
    #[doc = "Bit 28 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn setp28(&mut self) -> SETP28_W<28> {
        SETP28_W::new(self)
    }
    #[doc = "Bit 29 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn setp29(&mut self) -> SETP29_W<29> {
        SETP29_W::new(self)
    }
    #[doc = "Bit 30 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn setp30(&mut self) -> SETP30_W<30> {
        SETP30_W::new(self)
    }
    #[doc = "Bit 31 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    #[must_use]
    pub fn setp31(&mut self) -> SETP31_W<31> {
        SETP31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write: Set register for port 0/1 Read: output bits for port 0/1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set](index.html) module"]
pub struct SET_SPEC;
impl crate::RegisterSpec for SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [set::R](R) reader structure"]
impl crate::Readable for SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [set::W](W) writer structure"]
impl crate::Writable for SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SET%s to value 0"]
impl crate::Resettable for SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
