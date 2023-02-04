#[doc = "Register `CCR` reader"]
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR` writer"]
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAP0RE` reader - Capture on CT16B0_CAP0 rising edge: a sequence of 0 then 1 on CT16B0_CAP0 will cause CR0 to be loaded with the contents of TC."]
pub type CAP0RE_R = crate::BitReader<CAP0RE_A>;
#[doc = "Capture on CT16B0_CAP0 rising edge: a sequence of 0 then 1 on CT16B0_CAP0 will cause CR0 to be loaded with the contents of TC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAP0RE_A {
    #[doc = "1: Enabled."]
    ENABLED = 1,
    #[doc = "0: Disabled."]
    DISABLED = 0,
}
impl From<CAP0RE_A> for bool {
    #[inline(always)]
    fn from(variant: CAP0RE_A) -> Self {
        variant as u8 != 0
    }
}
impl CAP0RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAP0RE_A {
        match self.bits {
            true => CAP0RE_A::ENABLED,
            false => CAP0RE_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CAP0RE_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAP0RE_A::DISABLED
    }
}
#[doc = "Field `CAP0RE` writer - Capture on CT16B0_CAP0 rising edge: a sequence of 0 then 1 on CT16B0_CAP0 will cause CR0 to be loaded with the contents of TC."]
pub type CAP0RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, CAP0RE_A, O>;
impl<'a, const O: u8> CAP0RE_W<'a, O> {
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAP0RE_A::ENABLED)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAP0RE_A::DISABLED)
    }
}
#[doc = "Field `CAP0FE` reader - Capture on CT16B0_CAP0 falling edge: a sequence of 1 then 0 on CT16B0_CAP0 will cause CR0 to be loaded with the contents of TC."]
pub type CAP0FE_R = crate::BitReader<CAP0FE_A>;
#[doc = "Capture on CT16B0_CAP0 falling edge: a sequence of 1 then 0 on CT16B0_CAP0 will cause CR0 to be loaded with the contents of TC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAP0FE_A {
    #[doc = "1: Enabled."]
    ENABLED = 1,
    #[doc = "0: Disabled."]
    DISABLED = 0,
}
impl From<CAP0FE_A> for bool {
    #[inline(always)]
    fn from(variant: CAP0FE_A) -> Self {
        variant as u8 != 0
    }
}
impl CAP0FE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAP0FE_A {
        match self.bits {
            true => CAP0FE_A::ENABLED,
            false => CAP0FE_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CAP0FE_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAP0FE_A::DISABLED
    }
}
#[doc = "Field `CAP0FE` writer - Capture on CT16B0_CAP0 falling edge: a sequence of 1 then 0 on CT16B0_CAP0 will cause CR0 to be loaded with the contents of TC."]
pub type CAP0FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, CAP0FE_A, O>;
impl<'a, const O: u8> CAP0FE_W<'a, O> {
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAP0FE_A::ENABLED)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAP0FE_A::DISABLED)
    }
}
#[doc = "Field `CAP0I` reader - Interrupt on CT16B0_CAP0 event: a CR0 load due to a CT16B0_CAP0 event will generate an interrupt."]
pub type CAP0I_R = crate::BitReader<CAP0I_A>;
#[doc = "Interrupt on CT16B0_CAP0 event: a CR0 load due to a CT16B0_CAP0 event will generate an interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAP0I_A {
    #[doc = "1: Enabled."]
    ENABLED = 1,
    #[doc = "0: Disabled."]
    DISABLED = 0,
}
impl From<CAP0I_A> for bool {
    #[inline(always)]
    fn from(variant: CAP0I_A) -> Self {
        variant as u8 != 0
    }
}
impl CAP0I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAP0I_A {
        match self.bits {
            true => CAP0I_A::ENABLED,
            false => CAP0I_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CAP0I_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAP0I_A::DISABLED
    }
}
#[doc = "Field `CAP0I` writer - Interrupt on CT16B0_CAP0 event: a CR0 load due to a CT16B0_CAP0 event will generate an interrupt."]
pub type CAP0I_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, CAP0I_A, O>;
impl<'a, const O: u8> CAP0I_W<'a, O> {
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAP0I_A::ENABLED)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAP0I_A::DISABLED)
    }
}
#[doc = "Field `CAP1RE` reader - Capture on CT16B0_CAP1 rising edge: a sequence of 0 then 1 on CT16B0_CAP1 will cause CR1 to be loaded with the contents of TC. This bit is reserved for 16-bit timer1 CT16B1."]
pub type CAP1RE_R = crate::BitReader<CAP1RE_A>;
#[doc = "Capture on CT16B0_CAP1 rising edge: a sequence of 0 then 1 on CT16B0_CAP1 will cause CR1 to be loaded with the contents of TC. This bit is reserved for 16-bit timer1 CT16B1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAP1RE_A {
    #[doc = "1: Enabled."]
    ENABLED = 1,
    #[doc = "0: Disabled."]
    DISABLED = 0,
}
impl From<CAP1RE_A> for bool {
    #[inline(always)]
    fn from(variant: CAP1RE_A) -> Self {
        variant as u8 != 0
    }
}
impl CAP1RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAP1RE_A {
        match self.bits {
            true => CAP1RE_A::ENABLED,
            false => CAP1RE_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CAP1RE_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAP1RE_A::DISABLED
    }
}
#[doc = "Field `CAP1RE` writer - Capture on CT16B0_CAP1 rising edge: a sequence of 0 then 1 on CT16B0_CAP1 will cause CR1 to be loaded with the contents of TC. This bit is reserved for 16-bit timer1 CT16B1."]
pub type CAP1RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, CAP1RE_A, O>;
impl<'a, const O: u8> CAP1RE_W<'a, O> {
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAP1RE_A::ENABLED)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAP1RE_A::DISABLED)
    }
}
#[doc = "Field `CAP1FE` reader - Capture on CT16B0_CAP1 falling edge: a sequence of 1 then 0 on CT16B0_CAP1 will cause CR1 to be loaded with the contents of TC. This bit is reserved for 16-bit timer1 CT16B1."]
pub type CAP1FE_R = crate::BitReader<CAP1FE_A>;
#[doc = "Capture on CT16B0_CAP1 falling edge: a sequence of 1 then 0 on CT16B0_CAP1 will cause CR1 to be loaded with the contents of TC. This bit is reserved for 16-bit timer1 CT16B1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAP1FE_A {
    #[doc = "1: Enabled."]
    ENABLED = 1,
    #[doc = "0: Disabled."]
    DISABLED = 0,
}
impl From<CAP1FE_A> for bool {
    #[inline(always)]
    fn from(variant: CAP1FE_A) -> Self {
        variant as u8 != 0
    }
}
impl CAP1FE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAP1FE_A {
        match self.bits {
            true => CAP1FE_A::ENABLED,
            false => CAP1FE_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CAP1FE_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAP1FE_A::DISABLED
    }
}
#[doc = "Field `CAP1FE` writer - Capture on CT16B0_CAP1 falling edge: a sequence of 1 then 0 on CT16B0_CAP1 will cause CR1 to be loaded with the contents of TC. This bit is reserved for 16-bit timer1 CT16B1."]
pub type CAP1FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, CAP1FE_A, O>;
impl<'a, const O: u8> CAP1FE_W<'a, O> {
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAP1FE_A::ENABLED)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAP1FE_A::DISABLED)
    }
}
#[doc = "Field `CAP1I` reader - Interrupt on CT16B0_CAP1 event: a CR1 load due to a CT16B0_CAP1 event will generate an interrupt. This bit is reserved for 16-bit timer1 CT16B1."]
pub type CAP1I_R = crate::BitReader<CAP1I_A>;
#[doc = "Interrupt on CT16B0_CAP1 event: a CR1 load due to a CT16B0_CAP1 event will generate an interrupt. This bit is reserved for 16-bit timer1 CT16B1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAP1I_A {
    #[doc = "1: Enabled."]
    ENABLED = 1,
    #[doc = "0: Disabled."]
    DISABLED = 0,
}
impl From<CAP1I_A> for bool {
    #[inline(always)]
    fn from(variant: CAP1I_A) -> Self {
        variant as u8 != 0
    }
}
impl CAP1I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAP1I_A {
        match self.bits {
            true => CAP1I_A::ENABLED,
            false => CAP1I_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CAP1I_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAP1I_A::DISABLED
    }
}
#[doc = "Field `CAP1I` writer - Interrupt on CT16B0_CAP1 event: a CR1 load due to a CT16B0_CAP1 event will generate an interrupt. This bit is reserved for 16-bit timer1 CT16B1."]
pub type CAP1I_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, CAP1I_A, O>;
impl<'a, const O: u8> CAP1I_W<'a, O> {
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAP1I_A::ENABLED)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAP1I_A::DISABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Capture on CT16B0_CAP0 rising edge: a sequence of 0 then 1 on CT16B0_CAP0 will cause CR0 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn cap0re(&self) -> CAP0RE_R {
        CAP0RE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture on CT16B0_CAP0 falling edge: a sequence of 1 then 0 on CT16B0_CAP0 will cause CR0 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn cap0fe(&self) -> CAP0FE_R {
        CAP0FE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt on CT16B0_CAP0 event: a CR0 load due to a CT16B0_CAP0 event will generate an interrupt."]
    #[inline(always)]
    pub fn cap0i(&self) -> CAP0I_R {
        CAP0I_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Capture on CT16B0_CAP1 rising edge: a sequence of 0 then 1 on CT16B0_CAP1 will cause CR1 to be loaded with the contents of TC. This bit is reserved for 16-bit timer1 CT16B1."]
    #[inline(always)]
    pub fn cap1re(&self) -> CAP1RE_R {
        CAP1RE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Capture on CT16B0_CAP1 falling edge: a sequence of 1 then 0 on CT16B0_CAP1 will cause CR1 to be loaded with the contents of TC. This bit is reserved for 16-bit timer1 CT16B1."]
    #[inline(always)]
    pub fn cap1fe(&self) -> CAP1FE_R {
        CAP1FE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt on CT16B0_CAP1 event: a CR1 load due to a CT16B0_CAP1 event will generate an interrupt. This bit is reserved for 16-bit timer1 CT16B1."]
    #[inline(always)]
    pub fn cap1i(&self) -> CAP1I_R {
        CAP1I_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture on CT16B0_CAP0 rising edge: a sequence of 0 then 1 on CT16B0_CAP0 will cause CR0 to be loaded with the contents of TC."]
    #[inline(always)]
    #[must_use]
    pub fn cap0re(&mut self) -> CAP0RE_W<0> {
        CAP0RE_W::new(self)
    }
    #[doc = "Bit 1 - Capture on CT16B0_CAP0 falling edge: a sequence of 1 then 0 on CT16B0_CAP0 will cause CR0 to be loaded with the contents of TC."]
    #[inline(always)]
    #[must_use]
    pub fn cap0fe(&mut self) -> CAP0FE_W<1> {
        CAP0FE_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt on CT16B0_CAP0 event: a CR0 load due to a CT16B0_CAP0 event will generate an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cap0i(&mut self) -> CAP0I_W<2> {
        CAP0I_W::new(self)
    }
    #[doc = "Bit 6 - Capture on CT16B0_CAP1 rising edge: a sequence of 0 then 1 on CT16B0_CAP1 will cause CR1 to be loaded with the contents of TC. This bit is reserved for 16-bit timer1 CT16B1."]
    #[inline(always)]
    #[must_use]
    pub fn cap1re(&mut self) -> CAP1RE_W<6> {
        CAP1RE_W::new(self)
    }
    #[doc = "Bit 7 - Capture on CT16B0_CAP1 falling edge: a sequence of 1 then 0 on CT16B0_CAP1 will cause CR1 to be loaded with the contents of TC. This bit is reserved for 16-bit timer1 CT16B1."]
    #[inline(always)]
    #[must_use]
    pub fn cap1fe(&mut self) -> CAP1FE_W<7> {
        CAP1FE_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt on CT16B0_CAP1 event: a CR1 load due to a CT16B0_CAP1 event will generate an interrupt. This bit is reserved for 16-bit timer1 CT16B1."]
    #[inline(always)]
    #[must_use]
    pub fn cap1i(&mut self) -> CAP1I_W<8> {
        CAP1I_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated when a capture takes place.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](index.html) module"]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr::R](R) reader structure"]
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr::W](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
