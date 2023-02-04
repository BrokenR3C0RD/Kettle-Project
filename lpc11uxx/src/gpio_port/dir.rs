#[doc = "Register `DIR%s` reader"]
pub struct R(crate::R<DIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIR%s` writer"]
pub struct W(crate::W<DIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIR_SPEC>;
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
impl From<crate::W<DIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIRP0` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP0_R = crate::BitReader<bool>;
#[doc = "Field `DIRP0` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `DIRP1` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP1_R = crate::BitReader<bool>;
#[doc = "Field `DIRP1` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `DIRP2` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP2_R = crate::BitReader<bool>;
#[doc = "Field `DIRP2` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `DIRP3` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP3_R = crate::BitReader<bool>;
#[doc = "Field `DIRP3` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `DIRP4` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP4_R = crate::BitReader<bool>;
#[doc = "Field `DIRP4` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `DIRP5` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP5_R = crate::BitReader<bool>;
#[doc = "Field `DIRP5` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `DIRP6` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP6_R = crate::BitReader<bool>;
#[doc = "Field `DIRP6` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP6_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `DIRP7` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP7_R = crate::BitReader<bool>;
#[doc = "Field `DIRP7` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP7_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `DIRP8` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP8_R = crate::BitReader<bool>;
#[doc = "Field `DIRP8` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP8_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `DIRP9` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP9_R = crate::BitReader<bool>;
#[doc = "Field `DIRP9` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP9_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `DIRP10` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP10_R = crate::BitReader<bool>;
#[doc = "Field `DIRP10` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP10_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `DIRP11` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP11_R = crate::BitReader<bool>;
#[doc = "Field `DIRP11` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP11_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `DIRP12` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP12_R = crate::BitReader<bool>;
#[doc = "Field `DIRP12` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP12_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `DIRP13` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP13_R = crate::BitReader<bool>;
#[doc = "Field `DIRP13` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP13_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `DIRP14` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP14_R = crate::BitReader<bool>;
#[doc = "Field `DIRP14` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP14_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `DIRP15` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP15_R = crate::BitReader<bool>;
#[doc = "Field `DIRP15` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP15_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `DIRP16` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP16_R = crate::BitReader<bool>;
#[doc = "Field `DIRP16` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP16_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `DIRP17` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP17_R = crate::BitReader<bool>;
#[doc = "Field `DIRP17` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP17_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `DIRP18` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP18_R = crate::BitReader<bool>;
#[doc = "Field `DIRP18` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP18_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `DIRP19` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP19_R = crate::BitReader<bool>;
#[doc = "Field `DIRP19` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP19_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `DIRP20` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP20_R = crate::BitReader<bool>;
#[doc = "Field `DIRP20` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP20_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `DIRP21` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP21_R = crate::BitReader<bool>;
#[doc = "Field `DIRP21` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP21_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `DIRP22` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP22_R = crate::BitReader<bool>;
#[doc = "Field `DIRP22` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP22_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `DIRP23` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP23_R = crate::BitReader<bool>;
#[doc = "Field `DIRP23` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP23_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `DIRP24` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP24_R = crate::BitReader<bool>;
#[doc = "Field `DIRP24` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP24_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `DIRP25` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP25_R = crate::BitReader<bool>;
#[doc = "Field `DIRP25` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP25_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `DIRP26` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP26_R = crate::BitReader<bool>;
#[doc = "Field `DIRP26` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP26_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `DIRP27` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP27_R = crate::BitReader<bool>;
#[doc = "Field `DIRP27` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP27_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `DIRP28` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP28_R = crate::BitReader<bool>;
#[doc = "Field `DIRP28` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP28_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `DIRP29` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP29_R = crate::BitReader<bool>;
#[doc = "Field `DIRP29` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP29_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `DIRP30` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP30_R = crate::BitReader<bool>;
#[doc = "Field `DIRP30` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP30_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `DIRP31` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP31_R = crate::BitReader<bool>;
#[doc = "Field `DIRP31` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub type DIRP31_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp0(&self) -> DIRP0_R {
        DIRP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp1(&self) -> DIRP1_R {
        DIRP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp2(&self) -> DIRP2_R {
        DIRP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp3(&self) -> DIRP3_R {
        DIRP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp4(&self) -> DIRP4_R {
        DIRP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp5(&self) -> DIRP5_R {
        DIRP5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp6(&self) -> DIRP6_R {
        DIRP6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp7(&self) -> DIRP7_R {
        DIRP7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp8(&self) -> DIRP8_R {
        DIRP8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp9(&self) -> DIRP9_R {
        DIRP9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp10(&self) -> DIRP10_R {
        DIRP10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp11(&self) -> DIRP11_R {
        DIRP11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp12(&self) -> DIRP12_R {
        DIRP12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp13(&self) -> DIRP13_R {
        DIRP13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp14(&self) -> DIRP14_R {
        DIRP14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp15(&self) -> DIRP15_R {
        DIRP15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp16(&self) -> DIRP16_R {
        DIRP16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp17(&self) -> DIRP17_R {
        DIRP17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp18(&self) -> DIRP18_R {
        DIRP18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp19(&self) -> DIRP19_R {
        DIRP19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp20(&self) -> DIRP20_R {
        DIRP20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp21(&self) -> DIRP21_R {
        DIRP21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp22(&self) -> DIRP22_R {
        DIRP22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp23(&self) -> DIRP23_R {
        DIRP23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp24(&self) -> DIRP24_R {
        DIRP24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp25(&self) -> DIRP25_R {
        DIRP25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp26(&self) -> DIRP26_R {
        DIRP26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp27(&self) -> DIRP27_R {
        DIRP27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp28(&self) -> DIRP28_R {
        DIRP28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp29(&self) -> DIRP29_R {
        DIRP29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp30(&self) -> DIRP30_R {
        DIRP30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp31(&self) -> DIRP31_R {
        DIRP31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    #[must_use]
    pub fn dirp0(&mut self) -> DIRP0_W<0> {
        DIRP0_W::new(self)
    }
    #[doc = "Bit 1 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    #[must_use]
    pub fn dirp1(&mut self) -> DIRP1_W<1> {
        DIRP1_W::new(self)
    }
    #[doc = "Bit 2 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    #[must_use]
    pub fn dirp2(&mut self) -> DIRP2_W<2> {
        DIRP2_W::new(self)
    }
    #[doc = "Bit 3 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    #[must_use]
    pub fn dirp3(&mut self) -> DIRP3_W<3> {
        DIRP3_W::new(self)
    }
    #[doc = "Bit 4 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    #[must_use]
    pub fn dirp4(&mut self) -> DIRP4_W<4> {
        DIRP4_W::new(self)
    }
    #[doc = "Bit 5 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    #[must_use]
    pub fn dirp5(&mut self) -> DIRP5_W<5> {
        DIRP5_W::new(self)
    }
    #[doc = "Bit 6 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    #[must_use]
    pub fn dirp6(&mut self) -> DIRP6_W<6> {
        DIRP6_W::new(self)
    }
    #[doc = "Bit 7 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    #[must_use]
    pub fn dirp7(&mut self) -> DIRP7_W<7> {
        DIRP7_W::new(self)
    }
    #[doc = "Bit 8 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    #[must_use]
    pub fn dirp8(&mut self) -> DIRP8_W<8> {
        DIRP8_W::new(self)
    }
    #[doc = "Bit 9 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    #[must_use]
    pub fn dirp9(&mut self) -> DIRP9_W<9> {
        DIRP9_W::new(self)
    }
    #[doc = "Bit 10 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    #[must_use]
    pub fn dirp10(&mut self) -> DIRP10_W<10> {
        DIRP10_W::new(self)
    }
    #[doc = "Bit 11 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    #[must_use]
    pub fn dirp11(&mut self) -> DIRP11_W<11> {
        DIRP11_W::new(self)
    }
    #[doc = "Bit 12 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    #[must_use]
    pub fn dirp12(&mut self) -> DIRP12_W<12> {
        DIRP12_W::new(self)
    }
    #[doc = "Bit 13 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    #[must_use]
    pub fn dirp13(&mut self) -> DIRP13_W<13> {
        DIRP13_W::new(self)
    }
    #[doc = "Bit 14 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    #[must_use]
    pub fn dirp14(&mut self) -> DIRP14_W<14> {
        DIRP14_W::new(self)
    }
    #[doc = "Bit 15 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    #[must_use]
    pub fn dirp15(&mut self) -> DIRP15_W<15> {
        DIRP15_W::new(self)
    }
    #[doc = "Bit 16 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    #[must_use]
    pub fn dirp16(&mut self) -> DIRP16_W<16> {
        DIRP16_W::new(self)
    }
    #[doc = "Bit 17 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    #[must_use]
    pub fn dirp17(&mut self) -> DIRP17_W<17> {
        DIRP17_W::new(self)
    }
    #[doc = "Bit 18 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    #[must_use]
    pub fn dirp18(&mut self) -> DIRP18_W<18> {
        DIRP18_W::new(self)
    }
    #[doc = "Bit 19 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    #[must_use]
    pub fn dirp19(&mut self) -> DIRP19_W<19> {
        DIRP19_W::new(self)
    }
    #[doc = "Bit 20 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    #[must_use]
    pub fn dirp20(&mut self) -> DIRP20_W<20> {
        DIRP20_W::new(self)
    }
    #[doc = "Bit 21 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    #[must_use]
    pub fn dirp21(&mut self) -> DIRP21_W<21> {
        DIRP21_W::new(self)
    }
    #[doc = "Bit 22 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    #[must_use]
    pub fn dirp22(&mut self) -> DIRP22_W<22> {
        DIRP22_W::new(self)
    }
    #[doc = "Bit 23 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    #[must_use]
    pub fn dirp23(&mut self) -> DIRP23_W<23> {
        DIRP23_W::new(self)
    }
    #[doc = "Bit 24 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    #[must_use]
    pub fn dirp24(&mut self) -> DIRP24_W<24> {
        DIRP24_W::new(self)
    }
    #[doc = "Bit 25 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    #[must_use]
    pub fn dirp25(&mut self) -> DIRP25_W<25> {
        DIRP25_W::new(self)
    }
    #[doc = "Bit 26 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    #[must_use]
    pub fn dirp26(&mut self) -> DIRP26_W<26> {
        DIRP26_W::new(self)
    }
    #[doc = "Bit 27 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    #[must_use]
    pub fn dirp27(&mut self) -> DIRP27_W<27> {
        DIRP27_W::new(self)
    }
    #[doc = "Bit 28 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    #[must_use]
    pub fn dirp28(&mut self) -> DIRP28_W<28> {
        DIRP28_W::new(self)
    }
    #[doc = "Bit 29 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    #[must_use]
    pub fn dirp29(&mut self) -> DIRP29_W<29> {
        DIRP29_W::new(self)
    }
    #[doc = "Bit 30 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    #[must_use]
    pub fn dirp30(&mut self) -> DIRP30_W<30> {
        DIRP30_W::new(self)
    }
    #[doc = "Bit 31 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    #[must_use]
    pub fn dirp31(&mut self) -> DIRP31_W<31> {
        DIRP31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Direction registers port 0/1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir](index.html) module"]
pub struct DIR_SPEC;
impl crate::RegisterSpec for DIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dir::R](R) reader structure"]
impl crate::Readable for DIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dir::W](W) writer structure"]
impl crate::Writable for DIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIR%s to value 0"]
impl crate::Resettable for DIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
