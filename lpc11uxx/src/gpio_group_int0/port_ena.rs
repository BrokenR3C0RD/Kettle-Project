#[doc = "Register `PORT_ENA%s` reader"]
pub struct R(crate::R<PORT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PORT_ENA%s` writer"]
pub struct W(crate::W<PORT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORT_ENA_SPEC>;
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
impl From<crate::W<PORT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENA_0` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_0_R = crate::BitReader<bool>;
#[doc = "Field `ENA_0` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_ENA_SPEC, bool, O>;
#[doc = "Field `ENA_1` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_1_R = crate::BitReader<bool>;
#[doc = "Field `ENA_1` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_ENA_SPEC, bool, O>;
#[doc = "Field `ENA_2` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_2_R = crate::BitReader<bool>;
#[doc = "Field `ENA_2` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_ENA_SPEC, bool, O>;
#[doc = "Field `ENA_3` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_3_R = crate::BitReader<bool>;
#[doc = "Field `ENA_3` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_ENA_SPEC, bool, O>;
#[doc = "Field `ENA_4` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_4_R = crate::BitReader<bool>;
#[doc = "Field `ENA_4` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_ENA_SPEC, bool, O>;
#[doc = "Field `ENA_5` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_5_R = crate::BitReader<bool>;
#[doc = "Field `ENA_5` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_ENA_SPEC, bool, O>;
#[doc = "Field `ENA_6` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_6_R = crate::BitReader<bool>;
#[doc = "Field `ENA_6` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_ENA_SPEC, bool, O>;
#[doc = "Field `ENA_7` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_7_R = crate::BitReader<bool>;
#[doc = "Field `ENA_7` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_ENA_SPEC, bool, O>;
#[doc = "Field `ENA_8` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_8_R = crate::BitReader<bool>;
#[doc = "Field `ENA_8` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_ENA_SPEC, bool, O>;
#[doc = "Field `ENA_9` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_9_R = crate::BitReader<bool>;
#[doc = "Field `ENA_9` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_9_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_ENA_SPEC, bool, O>;
#[doc = "Field `ENA_10` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_10_R = crate::BitReader<bool>;
#[doc = "Field `ENA_10` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_ENA_SPEC, bool, O>;
#[doc = "Field `ENA_11` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_11_R = crate::BitReader<bool>;
#[doc = "Field `ENA_11` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_ENA_SPEC, bool, O>;
#[doc = "Field `ENA_12` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_12_R = crate::BitReader<bool>;
#[doc = "Field `ENA_12` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_12_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_ENA_SPEC, bool, O>;
#[doc = "Field `ENA_13` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_13_R = crate::BitReader<bool>;
#[doc = "Field `ENA_13` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_ENA_SPEC, bool, O>;
#[doc = "Field `ENA_14` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_14_R = crate::BitReader<bool>;
#[doc = "Field `ENA_14` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_ENA_SPEC, bool, O>;
#[doc = "Field `ENA_15` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_15_R = crate::BitReader<bool>;
#[doc = "Field `ENA_15` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_ENA_SPEC, bool, O>;
#[doc = "Field `ENA_16` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_16_R = crate::BitReader<bool>;
#[doc = "Field `ENA_16` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_16_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_ENA_SPEC, bool, O>;
#[doc = "Field `ENA_17` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_17_R = crate::BitReader<bool>;
#[doc = "Field `ENA_17` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_17_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_ENA_SPEC, bool, O>;
#[doc = "Field `ENA_18` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_18_R = crate::BitReader<bool>;
#[doc = "Field `ENA_18` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_18_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_ENA_SPEC, bool, O>;
#[doc = "Field `ENA_19` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_19_R = crate::BitReader<bool>;
#[doc = "Field `ENA_19` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_19_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_ENA_SPEC, bool, O>;
#[doc = "Field `ENA_20` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_20_R = crate::BitReader<bool>;
#[doc = "Field `ENA_20` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_20_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_ENA_SPEC, bool, O>;
#[doc = "Field `ENA_21` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_21_R = crate::BitReader<bool>;
#[doc = "Field `ENA_21` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_21_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_ENA_SPEC, bool, O>;
#[doc = "Field `ENA_22` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_22_R = crate::BitReader<bool>;
#[doc = "Field `ENA_22` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_22_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_ENA_SPEC, bool, O>;
#[doc = "Field `ENA_23` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_23_R = crate::BitReader<bool>;
#[doc = "Field `ENA_23` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_23_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_ENA_SPEC, bool, O>;
#[doc = "Field `ENA_24` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_24_R = crate::BitReader<bool>;
#[doc = "Field `ENA_24` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_24_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_ENA_SPEC, bool, O>;
#[doc = "Field `ENA_25` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_25_R = crate::BitReader<bool>;
#[doc = "Field `ENA_25` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_25_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_ENA_SPEC, bool, O>;
#[doc = "Field `ENA_26` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_26_R = crate::BitReader<bool>;
#[doc = "Field `ENA_26` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_26_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_ENA_SPEC, bool, O>;
#[doc = "Field `ENA_27` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_27_R = crate::BitReader<bool>;
#[doc = "Field `ENA_27` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_27_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_ENA_SPEC, bool, O>;
#[doc = "Field `ENA_28` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_28_R = crate::BitReader<bool>;
#[doc = "Field `ENA_28` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_28_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_ENA_SPEC, bool, O>;
#[doc = "Field `ENA_29` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_29_R = crate::BitReader<bool>;
#[doc = "Field `ENA_29` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_29_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_ENA_SPEC, bool, O>;
#[doc = "Field `ENA_30` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_30_R = crate::BitReader<bool>;
#[doc = "Field `ENA_30` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_30_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_ENA_SPEC, bool, O>;
#[doc = "Field `ENA_31` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_31_R = crate::BitReader<bool>;
#[doc = "Field `ENA_31` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_31_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_ENA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_0(&self) -> ENA_0_R {
        ENA_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_1(&self) -> ENA_1_R {
        ENA_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_2(&self) -> ENA_2_R {
        ENA_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_3(&self) -> ENA_3_R {
        ENA_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_4(&self) -> ENA_4_R {
        ENA_4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_5(&self) -> ENA_5_R {
        ENA_5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_6(&self) -> ENA_6_R {
        ENA_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_7(&self) -> ENA_7_R {
        ENA_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_8(&self) -> ENA_8_R {
        ENA_8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_9(&self) -> ENA_9_R {
        ENA_9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_10(&self) -> ENA_10_R {
        ENA_10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_11(&self) -> ENA_11_R {
        ENA_11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_12(&self) -> ENA_12_R {
        ENA_12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_13(&self) -> ENA_13_R {
        ENA_13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_14(&self) -> ENA_14_R {
        ENA_14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_15(&self) -> ENA_15_R {
        ENA_15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_16(&self) -> ENA_16_R {
        ENA_16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_17(&self) -> ENA_17_R {
        ENA_17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_18(&self) -> ENA_18_R {
        ENA_18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_19(&self) -> ENA_19_R {
        ENA_19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_20(&self) -> ENA_20_R {
        ENA_20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_21(&self) -> ENA_21_R {
        ENA_21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_22(&self) -> ENA_22_R {
        ENA_22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_23(&self) -> ENA_23_R {
        ENA_23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_24(&self) -> ENA_24_R {
        ENA_24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_25(&self) -> ENA_25_R {
        ENA_25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_26(&self) -> ENA_26_R {
        ENA_26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_27(&self) -> ENA_27_R {
        ENA_27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_28(&self) -> ENA_28_R {
        ENA_28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_29(&self) -> ENA_29_R {
        ENA_29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_30(&self) -> ENA_30_R {
        ENA_30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_31(&self) -> ENA_31_R {
        ENA_31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ena_0(&mut self) -> ENA_0_W<0> {
        ENA_0_W::new(self)
    }
    #[doc = "Bit 1 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ena_1(&mut self) -> ENA_1_W<1> {
        ENA_1_W::new(self)
    }
    #[doc = "Bit 2 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ena_2(&mut self) -> ENA_2_W<2> {
        ENA_2_W::new(self)
    }
    #[doc = "Bit 3 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ena_3(&mut self) -> ENA_3_W<3> {
        ENA_3_W::new(self)
    }
    #[doc = "Bit 4 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ena_4(&mut self) -> ENA_4_W<4> {
        ENA_4_W::new(self)
    }
    #[doc = "Bit 5 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ena_5(&mut self) -> ENA_5_W<5> {
        ENA_5_W::new(self)
    }
    #[doc = "Bit 6 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ena_6(&mut self) -> ENA_6_W<6> {
        ENA_6_W::new(self)
    }
    #[doc = "Bit 7 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ena_7(&mut self) -> ENA_7_W<7> {
        ENA_7_W::new(self)
    }
    #[doc = "Bit 8 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ena_8(&mut self) -> ENA_8_W<8> {
        ENA_8_W::new(self)
    }
    #[doc = "Bit 9 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ena_9(&mut self) -> ENA_9_W<9> {
        ENA_9_W::new(self)
    }
    #[doc = "Bit 10 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ena_10(&mut self) -> ENA_10_W<10> {
        ENA_10_W::new(self)
    }
    #[doc = "Bit 11 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ena_11(&mut self) -> ENA_11_W<11> {
        ENA_11_W::new(self)
    }
    #[doc = "Bit 12 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ena_12(&mut self) -> ENA_12_W<12> {
        ENA_12_W::new(self)
    }
    #[doc = "Bit 13 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ena_13(&mut self) -> ENA_13_W<13> {
        ENA_13_W::new(self)
    }
    #[doc = "Bit 14 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ena_14(&mut self) -> ENA_14_W<14> {
        ENA_14_W::new(self)
    }
    #[doc = "Bit 15 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ena_15(&mut self) -> ENA_15_W<15> {
        ENA_15_W::new(self)
    }
    #[doc = "Bit 16 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ena_16(&mut self) -> ENA_16_W<16> {
        ENA_16_W::new(self)
    }
    #[doc = "Bit 17 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ena_17(&mut self) -> ENA_17_W<17> {
        ENA_17_W::new(self)
    }
    #[doc = "Bit 18 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ena_18(&mut self) -> ENA_18_W<18> {
        ENA_18_W::new(self)
    }
    #[doc = "Bit 19 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ena_19(&mut self) -> ENA_19_W<19> {
        ENA_19_W::new(self)
    }
    #[doc = "Bit 20 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ena_20(&mut self) -> ENA_20_W<20> {
        ENA_20_W::new(self)
    }
    #[doc = "Bit 21 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ena_21(&mut self) -> ENA_21_W<21> {
        ENA_21_W::new(self)
    }
    #[doc = "Bit 22 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ena_22(&mut self) -> ENA_22_W<22> {
        ENA_22_W::new(self)
    }
    #[doc = "Bit 23 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ena_23(&mut self) -> ENA_23_W<23> {
        ENA_23_W::new(self)
    }
    #[doc = "Bit 24 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ena_24(&mut self) -> ENA_24_W<24> {
        ENA_24_W::new(self)
    }
    #[doc = "Bit 25 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ena_25(&mut self) -> ENA_25_W<25> {
        ENA_25_W::new(self)
    }
    #[doc = "Bit 26 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ena_26(&mut self) -> ENA_26_W<26> {
        ENA_26_W::new(self)
    }
    #[doc = "Bit 27 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ena_27(&mut self) -> ENA_27_W<27> {
        ENA_27_W::new(self)
    }
    #[doc = "Bit 28 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ena_28(&mut self) -> ENA_28_W<28> {
        ENA_28_W::new(self)
    }
    #[doc = "Bit 29 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ena_29(&mut self) -> ENA_29_W<29> {
        ENA_29_W::new(self)
    }
    #[doc = "Bit 30 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ena_30(&mut self) -> ENA_30_W<30> {
        ENA_30_W::new(self)
    }
    #[doc = "Bit 31 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ena_31(&mut self) -> ENA_31_W<31> {
        ENA_31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO grouped interrupt port 0/1 enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [port_ena](index.html) module"]
pub struct PORT_ENA_SPEC;
impl crate::RegisterSpec for PORT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [port_ena::R](R) reader structure"]
impl crate::Readable for PORT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [port_ena::W](W) writer structure"]
impl crate::Writable for PORT_ENA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PORT_ENA%s to value 0"]
impl crate::Resettable for PORT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
