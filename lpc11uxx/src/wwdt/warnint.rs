#[doc = "Register `WARNINT` reader"]
pub struct R(crate::R<WARNINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WARNINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WARNINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WARNINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WARNINT` writer"]
pub struct W(crate::W<WARNINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WARNINT_SPEC>;
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
impl From<crate::W<WARNINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WARNINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WARNINT` reader - Watchdog warning interrupt compare value."]
pub type WARNINT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WARNINT` writer - Watchdog warning interrupt compare value."]
pub type WARNINT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WARNINT_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Watchdog warning interrupt compare value."]
    #[inline(always)]
    pub fn warnint(&self) -> WARNINT_R {
        WARNINT_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Watchdog warning interrupt compare value."]
    #[inline(always)]
    #[must_use]
    pub fn warnint(&mut self) -> WARNINT_W<0> {
        WARNINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Warning Interrupt compare value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [warnint](index.html) module"]
pub struct WARNINT_SPEC;
impl crate::RegisterSpec for WARNINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [warnint::R](R) reader structure"]
impl crate::Readable for WARNINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [warnint::W](W) writer structure"]
impl crate::Writable for WARNINT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WARNINT to value 0"]
impl crate::Resettable for WARNINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
