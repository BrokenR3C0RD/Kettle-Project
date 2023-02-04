#[doc = "Register `CTCR` reader"]
pub struct R(crate::R<CTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTCR` writer"]
pub struct W(crate::W<CTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTCR_SPEC>;
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
impl From<crate::W<CTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTM` reader - Counter/Timer Mode. This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). If Counter mode is selected in the CTCR, bits 2:0 in the Capture Control Register (CCR) must be programmed as 000."]
pub type CTM_R = crate::FieldReader<u8, CTM_A>;
#[doc = "Counter/Timer Mode. This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). If Counter mode is selected in the CTCR, bits 2:0 in the Capture Control Register (CCR) must be programmed as 000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTM_A {
    #[doc = "0: Timer Mode: every rising PCLK edge"]
    TIMER = 0,
    #[doc = "1: Counter Mode: TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    COUNTER_RISING = 1,
    #[doc = "2: Counter Mode: TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    COUNTER_FALLING = 2,
    #[doc = "3: Counter Mode: TC is incremented on both edges on the CAP input selected by bits 3:2."]
    COUNTER_BOTH = 3,
}
impl From<CTM_A> for u8 {
    #[inline(always)]
    fn from(variant: CTM_A) -> Self {
        variant as _
    }
}
impl CTM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTM_A {
        match self.bits {
            0 => CTM_A::TIMER,
            1 => CTM_A::COUNTER_RISING,
            2 => CTM_A::COUNTER_FALLING,
            3 => CTM_A::COUNTER_BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER`"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == CTM_A::TIMER
    }
    #[doc = "Checks if the value of the field is `COUNTER_RISING`"]
    #[inline(always)]
    pub fn is_counter_rising(&self) -> bool {
        *self == CTM_A::COUNTER_RISING
    }
    #[doc = "Checks if the value of the field is `COUNTER_FALLING`"]
    #[inline(always)]
    pub fn is_counter_falling(&self) -> bool {
        *self == CTM_A::COUNTER_FALLING
    }
    #[doc = "Checks if the value of the field is `COUNTER_BOTH`"]
    #[inline(always)]
    pub fn is_counter_both(&self) -> bool {
        *self == CTM_A::COUNTER_BOTH
    }
}
#[doc = "Field `CTM` writer - Counter/Timer Mode. This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). If Counter mode is selected in the CTCR, bits 2:0 in the Capture Control Register (CCR) must be programmed as 000."]
pub type CTM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CTCR_SPEC, u8, CTM_A, 2, O>;
impl<'a, const O: u8> CTM_W<'a, O> {
    #[doc = "Timer Mode: every rising PCLK edge"]
    #[inline(always)]
    pub fn timer(self) -> &'a mut W {
        self.variant(CTM_A::TIMER)
    }
    #[doc = "Counter Mode: TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn counter_rising(self) -> &'a mut W {
        self.variant(CTM_A::COUNTER_RISING)
    }
    #[doc = "Counter Mode: TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn counter_falling(self) -> &'a mut W {
        self.variant(CTM_A::COUNTER_FALLING)
    }
    #[doc = "Counter Mode: TC is incremented on both edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn counter_both(self) -> &'a mut W {
        self.variant(CTM_A::COUNTER_BOTH)
    }
}
#[doc = "Field `CIS` reader - Count Input Select. In counter mode (when bits 1:0 in this register are not 00), these bits select which CAP pin or comparator output is sampled for clocking. Values 0x2 to 0x3 are reserved."]
pub type CIS_R = crate::FieldReader<u8, CIS_A>;
#[doc = "Count Input Select. In counter mode (when bits 1:0 in this register are not 00), these bits select which CAP pin or comparator output is sampled for clocking. Values 0x2 to 0x3 are reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CIS_A {
    #[doc = "0: CT16B1_CAP0."]
    CT16B1_CAP0 = 0,
    #[doc = "1: CT16B1_CAP1."]
    CT16B1_CAP1 = 1,
}
impl From<CIS_A> for u8 {
    #[inline(always)]
    fn from(variant: CIS_A) -> Self {
        variant as _
    }
}
impl CIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CIS_A> {
        match self.bits {
            0 => Some(CIS_A::CT16B1_CAP0),
            1 => Some(CIS_A::CT16B1_CAP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CT16B1_CAP0`"]
    #[inline(always)]
    pub fn is_ct16b1_cap0(&self) -> bool {
        *self == CIS_A::CT16B1_CAP0
    }
    #[doc = "Checks if the value of the field is `CT16B1_CAP1`"]
    #[inline(always)]
    pub fn is_ct16b1_cap1(&self) -> bool {
        *self == CIS_A::CT16B1_CAP1
    }
}
#[doc = "Field `CIS` writer - Count Input Select. In counter mode (when bits 1:0 in this register are not 00), these bits select which CAP pin or comparator output is sampled for clocking. Values 0x2 to 0x3 are reserved."]
pub type CIS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTCR_SPEC, u8, CIS_A, 2, O>;
impl<'a, const O: u8> CIS_W<'a, O> {
    #[doc = "CT16B1_CAP0."]
    #[inline(always)]
    pub fn ct16b1_cap0(self) -> &'a mut W {
        self.variant(CIS_A::CT16B1_CAP0)
    }
    #[doc = "CT16B1_CAP1."]
    #[inline(always)]
    pub fn ct16b1_cap1(self) -> &'a mut W {
        self.variant(CIS_A::CT16B1_CAP1)
    }
}
#[doc = "Field `ENCC` reader - Setting this bit to 1 enables clearing of the timer and the prescaler when the capture-edge event specified in bits 7:5 occurs."]
pub type ENCC_R = crate::BitReader<bool>;
#[doc = "Field `ENCC` writer - Setting this bit to 1 enables clearing of the timer and the prescaler when the capture-edge event specified in bits 7:5 occurs."]
pub type ENCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTCR_SPEC, bool, O>;
#[doc = "Field `SELCC` reader - When bit 4 is a 1, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is low. Values 0x6 to 0x7 are reserved."]
pub type SELCC_R = crate::FieldReader<u8, SELCC_A>;
#[doc = "When bit 4 is a 1, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is low. Values 0x6 to 0x7 are reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SELCC_A {
    #[doc = "0: Rising Edge of CT16B1_CAP0 clears the timer (if bit 4 is set)."]
    RISING_CAP0 = 0,
    #[doc = "1: Falling Edge of CT16B1_CAP0 clears the timer (if bit 4 is set)."]
    FALLING_CAP0 = 1,
    #[doc = "2: Rising Edge of CT16B1_CAP1 clears the timer (if bit 4 is set)."]
    RISING_CAP1 = 2,
    #[doc = "3: Falling Edge of CT16B1_CAP1 clears the timer (if bit 4 is set)."]
    FALLING_CAP1 = 3,
}
impl From<SELCC_A> for u8 {
    #[inline(always)]
    fn from(variant: SELCC_A) -> Self {
        variant as _
    }
}
impl SELCC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SELCC_A> {
        match self.bits {
            0 => Some(SELCC_A::RISING_CAP0),
            1 => Some(SELCC_A::FALLING_CAP0),
            2 => Some(SELCC_A::RISING_CAP1),
            3 => Some(SELCC_A::FALLING_CAP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RISING_CAP0`"]
    #[inline(always)]
    pub fn is_rising_cap0(&self) -> bool {
        *self == SELCC_A::RISING_CAP0
    }
    #[doc = "Checks if the value of the field is `FALLING_CAP0`"]
    #[inline(always)]
    pub fn is_falling_cap0(&self) -> bool {
        *self == SELCC_A::FALLING_CAP0
    }
    #[doc = "Checks if the value of the field is `RISING_CAP1`"]
    #[inline(always)]
    pub fn is_rising_cap1(&self) -> bool {
        *self == SELCC_A::RISING_CAP1
    }
    #[doc = "Checks if the value of the field is `FALLING_CAP1`"]
    #[inline(always)]
    pub fn is_falling_cap1(&self) -> bool {
        *self == SELCC_A::FALLING_CAP1
    }
}
#[doc = "Field `SELCC` writer - When bit 4 is a 1, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is low. Values 0x6 to 0x7 are reserved."]
pub type SELCC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTCR_SPEC, u8, SELCC_A, 3, O>;
impl<'a, const O: u8> SELCC_W<'a, O> {
    #[doc = "Rising Edge of CT16B1_CAP0 clears the timer (if bit 4 is set)."]
    #[inline(always)]
    pub fn rising_cap0(self) -> &'a mut W {
        self.variant(SELCC_A::RISING_CAP0)
    }
    #[doc = "Falling Edge of CT16B1_CAP0 clears the timer (if bit 4 is set)."]
    #[inline(always)]
    pub fn falling_cap0(self) -> &'a mut W {
        self.variant(SELCC_A::FALLING_CAP0)
    }
    #[doc = "Rising Edge of CT16B1_CAP1 clears the timer (if bit 4 is set)."]
    #[inline(always)]
    pub fn rising_cap1(self) -> &'a mut W {
        self.variant(SELCC_A::RISING_CAP1)
    }
    #[doc = "Falling Edge of CT16B1_CAP1 clears the timer (if bit 4 is set)."]
    #[inline(always)]
    pub fn falling_cap1(self) -> &'a mut W {
        self.variant(SELCC_A::FALLING_CAP1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Counter/Timer Mode. This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). If Counter mode is selected in the CTCR, bits 2:0 in the Capture Control Register (CCR) must be programmed as 000."]
    #[inline(always)]
    pub fn ctm(&self) -> CTM_R {
        CTM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Count Input Select. In counter mode (when bits 1:0 in this register are not 00), these bits select which CAP pin or comparator output is sampled for clocking. Values 0x2 to 0x3 are reserved."]
    #[inline(always)]
    pub fn cis(&self) -> CIS_R {
        CIS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Setting this bit to 1 enables clearing of the timer and the prescaler when the capture-edge event specified in bits 7:5 occurs."]
    #[inline(always)]
    pub fn encc(&self) -> ENCC_R {
        ENCC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - When bit 4 is a 1, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is low. Values 0x6 to 0x7 are reserved."]
    #[inline(always)]
    pub fn selcc(&self) -> SELCC_R {
        SELCC_R::new(((self.bits >> 5) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Counter/Timer Mode. This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). If Counter mode is selected in the CTCR, bits 2:0 in the Capture Control Register (CCR) must be programmed as 000."]
    #[inline(always)]
    #[must_use]
    pub fn ctm(&mut self) -> CTM_W<0> {
        CTM_W::new(self)
    }
    #[doc = "Bits 2:3 - Count Input Select. In counter mode (when bits 1:0 in this register are not 00), these bits select which CAP pin or comparator output is sampled for clocking. Values 0x2 to 0x3 are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn cis(&mut self) -> CIS_W<2> {
        CIS_W::new(self)
    }
    #[doc = "Bit 4 - Setting this bit to 1 enables clearing of the timer and the prescaler when the capture-edge event specified in bits 7:5 occurs."]
    #[inline(always)]
    #[must_use]
    pub fn encc(&mut self) -> ENCC_W<4> {
        ENCC_W::new(self)
    }
    #[doc = "Bits 5:7 - When bit 4 is a 1, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is low. Values 0x6 to 0x7 are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn selcc(&mut self) -> SELCC_W<5> {
        SELCC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Count Control Register. The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctcr](index.html) module"]
pub struct CTCR_SPEC;
impl crate::RegisterSpec for CTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctcr::R](R) reader structure"]
impl crate::Readable for CTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctcr::W](W) writer structure"]
impl crate::Writable for CTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTCR to value 0"]
impl crate::Resettable for CTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
