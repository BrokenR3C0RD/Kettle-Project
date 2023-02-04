#[doc = "Register `WDTOSCCTRL` reader"]
pub struct R(crate::R<WDTOSCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTOSCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTOSCCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTOSCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTOSCCTRL` writer"]
pub struct W(crate::W<WDTOSCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTOSCCTRL_SPEC>;
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
impl From<crate::W<WDTOSCCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTOSCCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIVSEL` reader - Select divider for Fclkana. wdt_osc_clk = Fclkana/ (2 x (1 + DIVSEL)) 00000: 2 x (1 + DIVSEL) = 2 00001: 2 x (1 + DIVSEL) = 4 to 11111: 2 x (1 + DIVSEL) = 64"]
pub type DIVSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIVSEL` writer - Select divider for Fclkana. wdt_osc_clk = Fclkana/ (2 x (1 + DIVSEL)) 00000: 2 x (1 + DIVSEL) = 2 00001: 2 x (1 + DIVSEL) = 4 to 11111: 2 x (1 + DIVSEL) = 64"]
pub type DIVSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WDTOSCCTRL_SPEC, u8, u8, 5, O>;
#[doc = "Field `FREQSEL` reader - Select watchdog oscillator analog output frequency (Fclkana)."]
pub type FREQSEL_R = crate::FieldReader<u8, FREQSEL_A>;
#[doc = "Select watchdog oscillator analog output frequency (Fclkana).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FREQSEL_A {
    #[doc = "1: 0.6 MHz"]
    _0_6_MHZ = 1,
    #[doc = "2: 1.05 MHz"]
    _1_05_MHZ = 2,
    #[doc = "3: 1.4 MHz"]
    _1_4_MHZ = 3,
    #[doc = "4: 1.75 MHz"]
    _1_75_MHZ = 4,
    #[doc = "5: 2.1 MHz"]
    _2_1_MHZ = 5,
    #[doc = "6: 2.4 MHz"]
    _2_4_MHZ = 6,
    #[doc = "7: 2.7 MHz"]
    _2_7_MHZ = 7,
    #[doc = "8: 3.0 MHz"]
    _3_0_MHZ = 8,
    #[doc = "9: 3.25 MHz"]
    _3_25_MHZ = 9,
    #[doc = "10: 3.5 MHz"]
    _3_5_MHZ = 10,
    #[doc = "11: 3.75 MHz"]
    _3_75_MHZ = 11,
    #[doc = "12: 4.0 MHz"]
    _4_0_MHZ = 12,
    #[doc = "13: 4.2 MHz"]
    _4_2_MHZ = 13,
    #[doc = "14: 4.4 MHz"]
    _4_4_MHZ = 14,
    #[doc = "15: 4.6 MHz"]
    _4_6_MHZ = 15,
}
impl From<FREQSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FREQSEL_A) -> Self {
        variant as _
    }
}
impl FREQSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FREQSEL_A> {
        match self.bits {
            1 => Some(FREQSEL_A::_0_6_MHZ),
            2 => Some(FREQSEL_A::_1_05_MHZ),
            3 => Some(FREQSEL_A::_1_4_MHZ),
            4 => Some(FREQSEL_A::_1_75_MHZ),
            5 => Some(FREQSEL_A::_2_1_MHZ),
            6 => Some(FREQSEL_A::_2_4_MHZ),
            7 => Some(FREQSEL_A::_2_7_MHZ),
            8 => Some(FREQSEL_A::_3_0_MHZ),
            9 => Some(FREQSEL_A::_3_25_MHZ),
            10 => Some(FREQSEL_A::_3_5_MHZ),
            11 => Some(FREQSEL_A::_3_75_MHZ),
            12 => Some(FREQSEL_A::_4_0_MHZ),
            13 => Some(FREQSEL_A::_4_2_MHZ),
            14 => Some(FREQSEL_A::_4_4_MHZ),
            15 => Some(FREQSEL_A::_4_6_MHZ),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0_6_MHZ`"]
    #[inline(always)]
    pub fn is_0_6_mhz(&self) -> bool {
        *self == FREQSEL_A::_0_6_MHZ
    }
    #[doc = "Checks if the value of the field is `_1_05_MHZ`"]
    #[inline(always)]
    pub fn is_1_05_mhz(&self) -> bool {
        *self == FREQSEL_A::_1_05_MHZ
    }
    #[doc = "Checks if the value of the field is `_1_4_MHZ`"]
    #[inline(always)]
    pub fn is_1_4_mhz(&self) -> bool {
        *self == FREQSEL_A::_1_4_MHZ
    }
    #[doc = "Checks if the value of the field is `_1_75_MHZ`"]
    #[inline(always)]
    pub fn is_1_75_mhz(&self) -> bool {
        *self == FREQSEL_A::_1_75_MHZ
    }
    #[doc = "Checks if the value of the field is `_2_1_MHZ`"]
    #[inline(always)]
    pub fn is_2_1_mhz(&self) -> bool {
        *self == FREQSEL_A::_2_1_MHZ
    }
    #[doc = "Checks if the value of the field is `_2_4_MHZ`"]
    #[inline(always)]
    pub fn is_2_4_mhz(&self) -> bool {
        *self == FREQSEL_A::_2_4_MHZ
    }
    #[doc = "Checks if the value of the field is `_2_7_MHZ`"]
    #[inline(always)]
    pub fn is_2_7_mhz(&self) -> bool {
        *self == FREQSEL_A::_2_7_MHZ
    }
    #[doc = "Checks if the value of the field is `_3_0_MHZ`"]
    #[inline(always)]
    pub fn is_3_0_mhz(&self) -> bool {
        *self == FREQSEL_A::_3_0_MHZ
    }
    #[doc = "Checks if the value of the field is `_3_25_MHZ`"]
    #[inline(always)]
    pub fn is_3_25_mhz(&self) -> bool {
        *self == FREQSEL_A::_3_25_MHZ
    }
    #[doc = "Checks if the value of the field is `_3_5_MHZ`"]
    #[inline(always)]
    pub fn is_3_5_mhz(&self) -> bool {
        *self == FREQSEL_A::_3_5_MHZ
    }
    #[doc = "Checks if the value of the field is `_3_75_MHZ`"]
    #[inline(always)]
    pub fn is_3_75_mhz(&self) -> bool {
        *self == FREQSEL_A::_3_75_MHZ
    }
    #[doc = "Checks if the value of the field is `_4_0_MHZ`"]
    #[inline(always)]
    pub fn is_4_0_mhz(&self) -> bool {
        *self == FREQSEL_A::_4_0_MHZ
    }
    #[doc = "Checks if the value of the field is `_4_2_MHZ`"]
    #[inline(always)]
    pub fn is_4_2_mhz(&self) -> bool {
        *self == FREQSEL_A::_4_2_MHZ
    }
    #[doc = "Checks if the value of the field is `_4_4_MHZ`"]
    #[inline(always)]
    pub fn is_4_4_mhz(&self) -> bool {
        *self == FREQSEL_A::_4_4_MHZ
    }
    #[doc = "Checks if the value of the field is `_4_6_MHZ`"]
    #[inline(always)]
    pub fn is_4_6_mhz(&self) -> bool {
        *self == FREQSEL_A::_4_6_MHZ
    }
}
#[doc = "Field `FREQSEL` writer - Select watchdog oscillator analog output frequency (Fclkana)."]
pub type FREQSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WDTOSCCTRL_SPEC, u8, FREQSEL_A, 4, O>;
impl<'a, const O: u8> FREQSEL_W<'a, O> {
    #[doc = "0.6 MHz"]
    #[inline(always)]
    pub fn _0_6_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_0_6_MHZ)
    }
    #[doc = "1.05 MHz"]
    #[inline(always)]
    pub fn _1_05_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_1_05_MHZ)
    }
    #[doc = "1.4 MHz"]
    #[inline(always)]
    pub fn _1_4_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_1_4_MHZ)
    }
    #[doc = "1.75 MHz"]
    #[inline(always)]
    pub fn _1_75_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_1_75_MHZ)
    }
    #[doc = "2.1 MHz"]
    #[inline(always)]
    pub fn _2_1_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_2_1_MHZ)
    }
    #[doc = "2.4 MHz"]
    #[inline(always)]
    pub fn _2_4_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_2_4_MHZ)
    }
    #[doc = "2.7 MHz"]
    #[inline(always)]
    pub fn _2_7_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_2_7_MHZ)
    }
    #[doc = "3.0 MHz"]
    #[inline(always)]
    pub fn _3_0_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_3_0_MHZ)
    }
    #[doc = "3.25 MHz"]
    #[inline(always)]
    pub fn _3_25_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_3_25_MHZ)
    }
    #[doc = "3.5 MHz"]
    #[inline(always)]
    pub fn _3_5_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_3_5_MHZ)
    }
    #[doc = "3.75 MHz"]
    #[inline(always)]
    pub fn _3_75_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_3_75_MHZ)
    }
    #[doc = "4.0 MHz"]
    #[inline(always)]
    pub fn _4_0_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_4_0_MHZ)
    }
    #[doc = "4.2 MHz"]
    #[inline(always)]
    pub fn _4_2_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_4_2_MHZ)
    }
    #[doc = "4.4 MHz"]
    #[inline(always)]
    pub fn _4_4_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_4_4_MHZ)
    }
    #[doc = "4.6 MHz"]
    #[inline(always)]
    pub fn _4_6_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_4_6_MHZ)
    }
}
impl R {
    #[doc = "Bits 0:4 - Select divider for Fclkana. wdt_osc_clk = Fclkana/ (2 x (1 + DIVSEL)) 00000: 2 x (1 + DIVSEL) = 2 00001: 2 x (1 + DIVSEL) = 4 to 11111: 2 x (1 + DIVSEL) = 64"]
    #[inline(always)]
    pub fn divsel(&self) -> DIVSEL_R {
        DIVSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:8 - Select watchdog oscillator analog output frequency (Fclkana)."]
    #[inline(always)]
    pub fn freqsel(&self) -> FREQSEL_R {
        FREQSEL_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select divider for Fclkana. wdt_osc_clk = Fclkana/ (2 x (1 + DIVSEL)) 00000: 2 x (1 + DIVSEL) = 2 00001: 2 x (1 + DIVSEL) = 4 to 11111: 2 x (1 + DIVSEL) = 64"]
    #[inline(always)]
    #[must_use]
    pub fn divsel(&mut self) -> DIVSEL_W<0> {
        DIVSEL_W::new(self)
    }
    #[doc = "Bits 5:8 - Select watchdog oscillator analog output frequency (Fclkana)."]
    #[inline(always)]
    #[must_use]
    pub fn freqsel(&mut self) -> FREQSEL_W<5> {
        FREQSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog oscillator control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtoscctrl](index.html) module"]
pub struct WDTOSCCTRL_SPEC;
impl crate::RegisterSpec for WDTOSCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdtoscctrl::R](R) reader structure"]
impl crate::Readable for WDTOSCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtoscctrl::W](W) writer structure"]
impl crate::Writable for WDTOSCCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDTOSCCTRL to value 0"]
impl crate::Resettable for WDTOSCCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
