#[doc = "Register `MAINCLKSEL` reader"]
pub struct R(crate::R<MAINCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAINCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAINCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAINCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAINCLKSEL` writer"]
pub struct W(crate::W<MAINCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAINCLKSEL_SPEC>;
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
impl From<crate::W<MAINCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAINCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - Clock source for main clock"]
pub type SEL_R = crate::FieldReader<u8, SEL_A>;
#[doc = "Clock source for main clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: IRC Oscillator"]
    IRC_OSCILLATOR = 0,
    #[doc = "1: PLL input"]
    PLL_INPUT = 1,
    #[doc = "2: Watchdog oscillator"]
    WATCHDOG_OSCILLATOR = 2,
    #[doc = "3: PLL output"]
    PLL_OUTPUT = 3,
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
            0 => SEL_A::IRC_OSCILLATOR,
            1 => SEL_A::PLL_INPUT,
            2 => SEL_A::WATCHDOG_OSCILLATOR,
            3 => SEL_A::PLL_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IRC_OSCILLATOR`"]
    #[inline(always)]
    pub fn is_irc_oscillator(&self) -> bool {
        *self == SEL_A::IRC_OSCILLATOR
    }
    #[doc = "Checks if the value of the field is `PLL_INPUT`"]
    #[inline(always)]
    pub fn is_pll_input(&self) -> bool {
        *self == SEL_A::PLL_INPUT
    }
    #[doc = "Checks if the value of the field is `WATCHDOG_OSCILLATOR`"]
    #[inline(always)]
    pub fn is_watchdog_oscillator(&self) -> bool {
        *self == SEL_A::WATCHDOG_OSCILLATOR
    }
    #[doc = "Checks if the value of the field is `PLL_OUTPUT`"]
    #[inline(always)]
    pub fn is_pll_output(&self) -> bool {
        *self == SEL_A::PLL_OUTPUT
    }
}
#[doc = "Field `SEL` writer - Clock source for main clock"]
pub type SEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MAINCLKSEL_SPEC, u8, SEL_A, 2, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "IRC Oscillator"]
    #[inline(always)]
    pub fn irc_oscillator(self) -> &'a mut W {
        self.variant(SEL_A::IRC_OSCILLATOR)
    }
    #[doc = "PLL input"]
    #[inline(always)]
    pub fn pll_input(self) -> &'a mut W {
        self.variant(SEL_A::PLL_INPUT)
    }
    #[doc = "Watchdog oscillator"]
    #[inline(always)]
    pub fn watchdog_oscillator(self) -> &'a mut W {
        self.variant(SEL_A::WATCHDOG_OSCILLATOR)
    }
    #[doc = "PLL output"]
    #[inline(always)]
    pub fn pll_output(self) -> &'a mut W {
        self.variant(SEL_A::PLL_OUTPUT)
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock source for main clock"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock source for main clock"]
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
#[doc = "Main clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mainclksel](index.html) module"]
pub struct MAINCLKSEL_SPEC;
impl crate::RegisterSpec for MAINCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mainclksel::R](R) reader structure"]
impl crate::Readable for MAINCLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mainclksel::W](W) writer structure"]
impl crate::Writable for MAINCLKSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAINCLKSEL to value 0"]
impl crate::Resettable for MAINCLKSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
