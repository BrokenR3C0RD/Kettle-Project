#[doc = "Register `PINTSEL%s` reader"]
pub struct R(crate::R<PINTSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINTSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINTSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINTSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINTSEL%s` writer"]
pub struct W(crate::W<PINTSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINTSEL_SPEC>;
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
impl From<crate::W<PINTSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINTSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTPIN` reader - Pin number select for pin interrupt. (PIO0_0 to PIO0_23 correspond to numbers 0 to 23 and PIO1_0 to PIO1_31 correspond to numbers 24 to 55)."]
pub type INTPIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INTPIN` writer - Pin number select for pin interrupt. (PIO0_0 to PIO0_23 correspond to numbers 0 to 23 and PIO1_0 to PIO1_31 correspond to numbers 24 to 55)."]
pub type INTPIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PINTSEL_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Pin number select for pin interrupt. (PIO0_0 to PIO0_23 correspond to numbers 0 to 23 and PIO1_0 to PIO1_31 correspond to numbers 24 to 55)."]
    #[inline(always)]
    pub fn intpin(&self) -> INTPIN_R {
        INTPIN_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Pin number select for pin interrupt. (PIO0_0 to PIO0_23 correspond to numbers 0 to 23 and PIO1_0 to PIO1_31 correspond to numbers 24 to 55)."]
    #[inline(always)]
    #[must_use]
    pub fn intpin(&mut self) -> INTPIN_W<0> {
        INTPIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Pin Interrupt Select register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pintsel](index.html) module"]
pub struct PINTSEL_SPEC;
impl crate::RegisterSpec for PINTSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pintsel::R](R) reader structure"]
impl crate::Readable for PINTSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pintsel::W](W) writer structure"]
impl crate::Writable for PINTSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINTSEL%s to value 0"]
impl crate::Resettable for PINTSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
