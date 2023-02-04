#[doc = "Register `HDEN` reader"]
pub struct R(crate::R<HDEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HDEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HDEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HDEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HDEN` writer"]
pub struct W(crate::W<HDEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HDEN_SPEC>;
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
impl From<crate::W<HDEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HDEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HDEN` reader - Half-duplex mode enable"]
pub type HDEN_R = crate::BitReader<HDEN_A>;
#[doc = "Half-duplex mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDEN_A {
    #[doc = "0: Disable half-duplex mode."]
    DISABLED = 0,
    #[doc = "1: Enable half-duplex mode."]
    ENABLED = 1,
}
impl From<HDEN_A> for bool {
    #[inline(always)]
    fn from(variant: HDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl HDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDEN_A {
        match self.bits {
            false => HDEN_A::DISABLED,
            true => HDEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HDEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HDEN_A::ENABLED
    }
}
#[doc = "Field `HDEN` writer - Half-duplex mode enable"]
pub type HDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HDEN_SPEC, HDEN_A, O>;
impl<'a, const O: u8> HDEN_W<'a, O> {
    #[doc = "Disable half-duplex mode."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HDEN_A::DISABLED)
    }
    #[doc = "Enable half-duplex mode."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HDEN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Half-duplex mode enable"]
    #[inline(always)]
    pub fn hden(&self) -> HDEN_R {
        HDEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Half-duplex mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn hden(&mut self) -> HDEN_W<0> {
        HDEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Half duplex enable register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hden](index.html) module"]
pub struct HDEN_SPEC;
impl crate::RegisterSpec for HDEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hden::R](R) reader structure"]
impl crate::Readable for HDEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hden::W](W) writer structure"]
impl crate::Writable for HDEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HDEN to value 0"]
impl crate::Resettable for HDEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
