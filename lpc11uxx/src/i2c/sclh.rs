#[doc = "Register `SCLH` reader"]
pub struct R(crate::R<SCLH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCLH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCLH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCLH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCLH` writer"]
pub struct W(crate::W<SCLH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCLH_SPEC>;
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
impl From<crate::W<SCLH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCLH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCLH` reader - Count for SCL HIGH time period selection."]
pub type SCLH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SCLH` writer - Count for SCL HIGH time period selection."]
pub type SCLH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCLH_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Count for SCL HIGH time period selection."]
    #[inline(always)]
    pub fn sclh(&self) -> SCLH_R {
        SCLH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Count for SCL HIGH time period selection."]
    #[inline(always)]
    #[must_use]
    pub fn sclh(&mut self) -> SCLH_W<0> {
        SCLH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCH Duty Cycle Register High Half Word. Determines the high time of the I2C clock.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sclh](index.html) module"]
pub struct SCLH_SPEC;
impl crate::RegisterSpec for SCLH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sclh::R](R) reader structure"]
impl crate::Readable for SCLH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sclh::W](W) writer structure"]
impl crate::Writable for SCLH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCLH to value 0x04"]
impl crate::Resettable for SCLH_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
