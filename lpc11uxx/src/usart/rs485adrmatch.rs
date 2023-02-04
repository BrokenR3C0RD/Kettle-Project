#[doc = "Register `RS485ADRMATCH` reader"]
pub struct R(crate::R<RS485ADRMATCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RS485ADRMATCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RS485ADRMATCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RS485ADRMATCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RS485ADRMATCH` writer"]
pub struct W(crate::W<RS485ADRMATCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RS485ADRMATCH_SPEC>;
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
impl From<crate::W<RS485ADRMATCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RS485ADRMATCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADRMATCH` reader - Contains the address match value."]
pub type ADRMATCH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADRMATCH` writer - Contains the address match value."]
pub type ADRMATCH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RS485ADRMATCH_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Contains the address match value."]
    #[inline(always)]
    pub fn adrmatch(&self) -> ADRMATCH_R {
        ADRMATCH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Contains the address match value."]
    #[inline(always)]
    #[must_use]
    pub fn adrmatch(&mut self) -> ADRMATCH_W<0> {
        ADRMATCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RS-485/EIA-485 address match. Contains the address match value for RS-485/EIA-485 mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rs485adrmatch](index.html) module"]
pub struct RS485ADRMATCH_SPEC;
impl crate::RegisterSpec for RS485ADRMATCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rs485adrmatch::R](R) reader structure"]
impl crate::Readable for RS485ADRMATCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rs485adrmatch::W](W) writer structure"]
impl crate::Writable for RS485ADRMATCH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RS485ADRMATCH to value 0"]
impl crate::Resettable for RS485ADRMATCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
