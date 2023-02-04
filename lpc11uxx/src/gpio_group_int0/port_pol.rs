#[doc = "Register `PORT_POL%s` reader"]
pub struct R(crate::R<PORT_POL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORT_POL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORT_POL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORT_POL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PORT_POL%s` writer"]
pub struct W(crate::W<PORT_POL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORT_POL_SPEC>;
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
impl From<crate::W<PORT_POL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORT_POL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POL_0` reader - Configure pin polarity of port pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1 . 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_0_R = crate::BitReader<bool>;
#[doc = "Field `POL_0` writer - Configure pin polarity of port pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1 . 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_POL_SPEC, bool, O>;
#[doc = "Field `POL_1` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_1_R = crate::BitReader<bool>;
#[doc = "Field `POL_1` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_POL_SPEC, bool, O>;
#[doc = "Field `POL_2` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_2_R = crate::BitReader<bool>;
#[doc = "Field `POL_2` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_POL_SPEC, bool, O>;
#[doc = "Field `POL_3` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_3_R = crate::BitReader<bool>;
#[doc = "Field `POL_3` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_POL_SPEC, bool, O>;
#[doc = "Field `POL_4` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_4_R = crate::BitReader<bool>;
#[doc = "Field `POL_4` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_POL_SPEC, bool, O>;
#[doc = "Field `POL_5` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_5_R = crate::BitReader<bool>;
#[doc = "Field `POL_5` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_POL_SPEC, bool, O>;
#[doc = "Field `POL_6` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_6_R = crate::BitReader<bool>;
#[doc = "Field `POL_6` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_POL_SPEC, bool, O>;
#[doc = "Field `POL_7` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_7_R = crate::BitReader<bool>;
#[doc = "Field `POL_7` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_POL_SPEC, bool, O>;
#[doc = "Field `POL_8` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_8_R = crate::BitReader<bool>;
#[doc = "Field `POL_8` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_POL_SPEC, bool, O>;
#[doc = "Field `POL_9` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_9_R = crate::BitReader<bool>;
#[doc = "Field `POL_9` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_9_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_POL_SPEC, bool, O>;
#[doc = "Field `POL_10` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_10_R = crate::BitReader<bool>;
#[doc = "Field `POL_10` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_POL_SPEC, bool, O>;
#[doc = "Field `POL_11` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_11_R = crate::BitReader<bool>;
#[doc = "Field `POL_11` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_POL_SPEC, bool, O>;
#[doc = "Field `POL_12` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_12_R = crate::BitReader<bool>;
#[doc = "Field `POL_12` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_12_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_POL_SPEC, bool, O>;
#[doc = "Field `POL_13` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_13_R = crate::BitReader<bool>;
#[doc = "Field `POL_13` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_POL_SPEC, bool, O>;
#[doc = "Field `POL_14` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_14_R = crate::BitReader<bool>;
#[doc = "Field `POL_14` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_POL_SPEC, bool, O>;
#[doc = "Field `POL_15` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_15_R = crate::BitReader<bool>;
#[doc = "Field `POL_15` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_POL_SPEC, bool, O>;
#[doc = "Field `POL_16` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_16_R = crate::BitReader<bool>;
#[doc = "Field `POL_16` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_16_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_POL_SPEC, bool, O>;
#[doc = "Field `POL_17` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_17_R = crate::BitReader<bool>;
#[doc = "Field `POL_17` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_17_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_POL_SPEC, bool, O>;
#[doc = "Field `POL_18` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_18_R = crate::BitReader<bool>;
#[doc = "Field `POL_18` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_18_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_POL_SPEC, bool, O>;
#[doc = "Field `POL_19` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_19_R = crate::BitReader<bool>;
#[doc = "Field `POL_19` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_19_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_POL_SPEC, bool, O>;
#[doc = "Field `POL_20` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_20_R = crate::BitReader<bool>;
#[doc = "Field `POL_20` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_20_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_POL_SPEC, bool, O>;
#[doc = "Field `POL_21` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_21_R = crate::BitReader<bool>;
#[doc = "Field `POL_21` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_21_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_POL_SPEC, bool, O>;
#[doc = "Field `POL_22` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_22_R = crate::BitReader<bool>;
#[doc = "Field `POL_22` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_22_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_POL_SPEC, bool, O>;
#[doc = "Field `POL_23` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_23_R = crate::BitReader<bool>;
#[doc = "Field `POL_23` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_23_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_POL_SPEC, bool, O>;
#[doc = "Field `POL_24` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_24_R = crate::BitReader<bool>;
#[doc = "Field `POL_24` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_24_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_POL_SPEC, bool, O>;
#[doc = "Field `POL_25` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_25_R = crate::BitReader<bool>;
#[doc = "Field `POL_25` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_25_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_POL_SPEC, bool, O>;
#[doc = "Field `POL_26` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_26_R = crate::BitReader<bool>;
#[doc = "Field `POL_26` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_26_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_POL_SPEC, bool, O>;
#[doc = "Field `POL_27` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_27_R = crate::BitReader<bool>;
#[doc = "Field `POL_27` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_27_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_POL_SPEC, bool, O>;
#[doc = "Field `POL_28` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_28_R = crate::BitReader<bool>;
#[doc = "Field `POL_28` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_28_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_POL_SPEC, bool, O>;
#[doc = "Field `POL_29` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_29_R = crate::BitReader<bool>;
#[doc = "Field `POL_29` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_29_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_POL_SPEC, bool, O>;
#[doc = "Field `POL_30` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_30_R = crate::BitReader<bool>;
#[doc = "Field `POL_30` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_30_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_POL_SPEC, bool, O>;
#[doc = "Field `POL_31` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_31_R = crate::BitReader<bool>;
#[doc = "Field `POL_31` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_31_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_POL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Configure pin polarity of port pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1 . 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_0(&self) -> POL_0_R {
        POL_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_1(&self) -> POL_1_R {
        POL_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_2(&self) -> POL_2_R {
        POL_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_3(&self) -> POL_3_R {
        POL_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_4(&self) -> POL_4_R {
        POL_4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_5(&self) -> POL_5_R {
        POL_5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_6(&self) -> POL_6_R {
        POL_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_7(&self) -> POL_7_R {
        POL_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_8(&self) -> POL_8_R {
        POL_8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_9(&self) -> POL_9_R {
        POL_9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_10(&self) -> POL_10_R {
        POL_10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_11(&self) -> POL_11_R {
        POL_11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_12(&self) -> POL_12_R {
        POL_12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_13(&self) -> POL_13_R {
        POL_13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_14(&self) -> POL_14_R {
        POL_14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_15(&self) -> POL_15_R {
        POL_15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_16(&self) -> POL_16_R {
        POL_16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_17(&self) -> POL_17_R {
        POL_17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_18(&self) -> POL_18_R {
        POL_18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_19(&self) -> POL_19_R {
        POL_19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_20(&self) -> POL_20_R {
        POL_20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_21(&self) -> POL_21_R {
        POL_21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_22(&self) -> POL_22_R {
        POL_22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_23(&self) -> POL_23_R {
        POL_23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_24(&self) -> POL_24_R {
        POL_24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_25(&self) -> POL_25_R {
        POL_25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_26(&self) -> POL_26_R {
        POL_26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_27(&self) -> POL_27_R {
        POL_27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_28(&self) -> POL_28_R {
        POL_28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_29(&self) -> POL_29_R {
        POL_29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_30(&self) -> POL_30_R {
        POL_30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_31(&self) -> POL_31_R {
        POL_31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configure pin polarity of port pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1 . 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pol_0(&mut self) -> POL_0_W<0> {
        POL_0_W::new(self)
    }
    #[doc = "Bit 1 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pol_1(&mut self) -> POL_1_W<1> {
        POL_1_W::new(self)
    }
    #[doc = "Bit 2 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pol_2(&mut self) -> POL_2_W<2> {
        POL_2_W::new(self)
    }
    #[doc = "Bit 3 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pol_3(&mut self) -> POL_3_W<3> {
        POL_3_W::new(self)
    }
    #[doc = "Bit 4 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pol_4(&mut self) -> POL_4_W<4> {
        POL_4_W::new(self)
    }
    #[doc = "Bit 5 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pol_5(&mut self) -> POL_5_W<5> {
        POL_5_W::new(self)
    }
    #[doc = "Bit 6 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pol_6(&mut self) -> POL_6_W<6> {
        POL_6_W::new(self)
    }
    #[doc = "Bit 7 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pol_7(&mut self) -> POL_7_W<7> {
        POL_7_W::new(self)
    }
    #[doc = "Bit 8 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pol_8(&mut self) -> POL_8_W<8> {
        POL_8_W::new(self)
    }
    #[doc = "Bit 9 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pol_9(&mut self) -> POL_9_W<9> {
        POL_9_W::new(self)
    }
    #[doc = "Bit 10 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pol_10(&mut self) -> POL_10_W<10> {
        POL_10_W::new(self)
    }
    #[doc = "Bit 11 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pol_11(&mut self) -> POL_11_W<11> {
        POL_11_W::new(self)
    }
    #[doc = "Bit 12 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pol_12(&mut self) -> POL_12_W<12> {
        POL_12_W::new(self)
    }
    #[doc = "Bit 13 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pol_13(&mut self) -> POL_13_W<13> {
        POL_13_W::new(self)
    }
    #[doc = "Bit 14 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pol_14(&mut self) -> POL_14_W<14> {
        POL_14_W::new(self)
    }
    #[doc = "Bit 15 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pol_15(&mut self) -> POL_15_W<15> {
        POL_15_W::new(self)
    }
    #[doc = "Bit 16 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pol_16(&mut self) -> POL_16_W<16> {
        POL_16_W::new(self)
    }
    #[doc = "Bit 17 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pol_17(&mut self) -> POL_17_W<17> {
        POL_17_W::new(self)
    }
    #[doc = "Bit 18 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pol_18(&mut self) -> POL_18_W<18> {
        POL_18_W::new(self)
    }
    #[doc = "Bit 19 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pol_19(&mut self) -> POL_19_W<19> {
        POL_19_W::new(self)
    }
    #[doc = "Bit 20 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pol_20(&mut self) -> POL_20_W<20> {
        POL_20_W::new(self)
    }
    #[doc = "Bit 21 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pol_21(&mut self) -> POL_21_W<21> {
        POL_21_W::new(self)
    }
    #[doc = "Bit 22 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pol_22(&mut self) -> POL_22_W<22> {
        POL_22_W::new(self)
    }
    #[doc = "Bit 23 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pol_23(&mut self) -> POL_23_W<23> {
        POL_23_W::new(self)
    }
    #[doc = "Bit 24 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pol_24(&mut self) -> POL_24_W<24> {
        POL_24_W::new(self)
    }
    #[doc = "Bit 25 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pol_25(&mut self) -> POL_25_W<25> {
        POL_25_W::new(self)
    }
    #[doc = "Bit 26 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pol_26(&mut self) -> POL_26_W<26> {
        POL_26_W::new(self)
    }
    #[doc = "Bit 27 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pol_27(&mut self) -> POL_27_W<27> {
        POL_27_W::new(self)
    }
    #[doc = "Bit 28 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pol_28(&mut self) -> POL_28_W<28> {
        POL_28_W::new(self)
    }
    #[doc = "Bit 29 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pol_29(&mut self) -> POL_29_W<29> {
        POL_29_W::new(self)
    }
    #[doc = "Bit 30 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pol_30(&mut self) -> POL_30_W<30> {
        POL_30_W::new(self)
    }
    #[doc = "Bit 31 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pol_31(&mut self) -> POL_31_W<31> {
        POL_31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO grouped interrupt port 0 polarity register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [port_pol](index.html) module"]
pub struct PORT_POL_SPEC;
impl crate::RegisterSpec for PORT_POL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [port_pol::R](R) reader structure"]
impl crate::Readable for PORT_POL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [port_pol::W](W) writer structure"]
impl crate::Writable for PORT_POL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PORT_POL%s to value 0xffff_ffff"]
impl crate::Resettable for PORT_POL_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
