#[doc = "Register `SYSPLLCLKSEL` reader"]
pub struct R(crate::R<SYSPLLCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSPLLCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSPLLCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSPLLCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSPLLCLKSEL` writer"]
pub struct W(crate::W<SYSPLLCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSPLLCLKSEL_SPEC>;
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
impl From<crate::W<SYSPLLCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSPLLCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - System PLL clock source"]
pub type SEL_R = crate::FieldReader<u8, SEL_A>;
#[doc = "System PLL clock source\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: IRC"]
    IRC = 0,
    #[doc = "1: Crystal Oscillator (SYSOSC)"]
    CRYSTAL_OSCILLATOR = 1,
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
            1 => SEL_A::CRYSTAL_OSCILLATOR,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IRC`"]
    #[inline(always)]
    pub fn is_irc(&self) -> bool {
        *self == SEL_A::IRC
    }
    #[doc = "Checks if the value of the field is `CRYSTAL_OSCILLATOR`"]
    #[inline(always)]
    pub fn is_crystal_oscillator(&self) -> bool {
        *self == SEL_A::CRYSTAL_OSCILLATOR
    }
}
#[doc = "Field `SEL` writer - System PLL clock source"]
pub type SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSPLLCLKSEL_SPEC, u8, SEL_A, 2, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "IRC"]
    #[inline(always)]
    pub fn irc(self) -> &'a mut W {
        self.variant(SEL_A::IRC)
    }
    #[doc = "Crystal Oscillator (SYSOSC)"]
    #[inline(always)]
    pub fn crystal_oscillator(self) -> &'a mut W {
        self.variant(SEL_A::CRYSTAL_OSCILLATOR)
    }
}
impl R {
    #[doc = "Bits 0:1 - System PLL clock source"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - System PLL clock source"]
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
#[doc = "System PLL clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syspllclksel](index.html) module"]
pub struct SYSPLLCLKSEL_SPEC;
impl crate::RegisterSpec for SYSPLLCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syspllclksel::R](R) reader structure"]
impl crate::Readable for SYSPLLCLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syspllclksel::W](W) writer structure"]
impl crate::Writable for SYSPLLCLKSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSPLLCLKSEL to value 0x01"]
impl crate::Resettable for SYSPLLCLKSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
