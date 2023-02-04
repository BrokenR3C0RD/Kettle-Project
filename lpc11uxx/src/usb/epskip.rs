#[doc = "Register `EPSKIP` reader"]
pub struct R(crate::R<EPSKIP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPSKIP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPSKIP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPSKIP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EPSKIP` writer"]
pub struct W(crate::W<EPSKIP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPSKIP_SPEC>;
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
impl From<crate::W<EPSKIP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPSKIP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SKIP` reader - Endpoint skip: Writing 1 to one of these bits, will indicate to HW that it must deactivate the buffer assigned to this endpoint and return control back to software. When HW has deactivated the endpoint, it will clear this bit, but it will not modify the EPINUSE bit. An interrupt will be generated when the Active bit goes from 1 to 0. Note: In case of double-buffering, HW will only clear the Active bit of the buffer indicated by the EPINUSE bit."]
pub type SKIP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SKIP` writer - Endpoint skip: Writing 1 to one of these bits, will indicate to HW that it must deactivate the buffer assigned to this endpoint and return control back to software. When HW has deactivated the endpoint, it will clear this bit, but it will not modify the EPINUSE bit. An interrupt will be generated when the Active bit goes from 1 to 0. Note: In case of double-buffering, HW will only clear the Active bit of the buffer indicated by the EPINUSE bit."]
pub type SKIP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EPSKIP_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 0:29 - Endpoint skip: Writing 1 to one of these bits, will indicate to HW that it must deactivate the buffer assigned to this endpoint and return control back to software. When HW has deactivated the endpoint, it will clear this bit, but it will not modify the EPINUSE bit. An interrupt will be generated when the Active bit goes from 1 to 0. Note: In case of double-buffering, HW will only clear the Active bit of the buffer indicated by the EPINUSE bit."]
    #[inline(always)]
    pub fn skip(&self) -> SKIP_R {
        SKIP_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Endpoint skip: Writing 1 to one of these bits, will indicate to HW that it must deactivate the buffer assigned to this endpoint and return control back to software. When HW has deactivated the endpoint, it will clear this bit, but it will not modify the EPINUSE bit. An interrupt will be generated when the Active bit goes from 1 to 0. Note: In case of double-buffering, HW will only clear the Active bit of the buffer indicated by the EPINUSE bit."]
    #[inline(always)]
    #[must_use]
    pub fn skip(&mut self) -> SKIP_W<0> {
        SKIP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Endpoint skip\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epskip](index.html) module"]
pub struct EPSKIP_SPEC;
impl crate::RegisterSpec for EPSKIP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [epskip::R](R) reader structure"]
impl crate::Readable for EPSKIP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [epskip::W](W) writer structure"]
impl crate::Writable for EPSKIP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EPSKIP to value 0"]
impl crate::Resettable for EPSKIP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
