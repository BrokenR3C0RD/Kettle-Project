#[doc = "Register `USBPLLCLKSEL` reader"]
pub struct R(crate::R<USBPLLCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBPLLCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBPLLCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBPLLCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBPLLCLKSEL` writer"]
pub struct W(crate::W<USBPLLCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBPLLCLKSEL_SPEC>;
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
impl From<crate::W<USBPLLCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBPLLCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - USB PLL clock source"]
pub type SEL_R = crate::FieldReader<u8, SEL_A>;
#[doc = "USB PLL clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: IRC. The USB PLL clock source must be switched to system oscillator for correct full-speed USB operation. The IRC is suitable for low-speed USB operation."]
    IRC = 0,
    #[doc = "1: System oscillator"]
    SYSOSC = 1,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
impl SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            0 => SEL_A::IRC,
            1 => SEL_A::SYSOSC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IRC`"]
    #[inline(always)]
    pub fn is_irc(&self) -> bool {
        *self == SEL_A::IRC
    }
    #[doc = "Checks if the value of the field is `SYSOSC`"]
    #[inline(always)]
    pub fn is_sysosc(&self) -> bool {
        *self == SEL_A::SYSOSC
    }
}
#[doc = "Field `SEL` writer - USB PLL clock source"]
pub type SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USBPLLCLKSEL_SPEC, u8, SEL_A, 2, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "IRC. The USB PLL clock source must be switched to system oscillator for correct full-speed USB operation. The IRC is suitable for low-speed USB operation."]
    #[inline(always)]
    pub fn irc(self) -> &'a mut W {
        self.variant(SEL_A::IRC)
    }
    #[doc = "System oscillator"]
    #[inline(always)]
    pub fn sysosc(self) -> &'a mut W {
        self.variant(SEL_A::SYSOSC)
    }
}
impl R {
    #[doc = "Bits 0:1 - USB PLL clock source"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - USB PLL clock source"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<0> {
        SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB PLL clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbpllclksel](index.html) module"]
pub struct USBPLLCLKSEL_SPEC;
impl crate::RegisterSpec for USBPLLCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbpllclksel::R](R) reader structure"]
impl crate::Readable for USBPLLCLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbpllclksel::W](W) writer structure"]
impl crate::Writable for USBPLLCLKSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBPLLCLKSEL to value 0"]
impl crate::Resettable for USBPLLCLKSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
