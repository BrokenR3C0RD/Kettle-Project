#[doc = "Register `SYSTCKCAL` reader"]
pub struct R(crate::R<SYSTCKCAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSTCKCAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSTCKCAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSTCKCAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSTCKCAL` writer"]
pub struct W(crate::W<SYSTCKCAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSTCKCAL_SPEC>;
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
impl From<crate::W<SYSTCKCAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSTCKCAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAL` reader - System tick timer calibration value"]
pub type CAL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CAL` writer - System tick timer calibration value"]
pub type CAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSTCKCAL_SPEC, u32, u32, 26, O>;
impl R {
    #[doc = "Bits 0:25 - System tick timer calibration value"]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - System tick timer calibration value"]
    #[inline(always)]
    #[must_use]
    pub fn cal(&mut self) -> CAL_W<0> {
        CAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System tick counter calibration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [systckcal](index.html) module"]
pub struct SYSTCKCAL_SPEC;
impl crate::RegisterSpec for SYSTCKCAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [systckcal::R](R) reader structure"]
impl crate::Readable for SYSTCKCAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [systckcal::W](W) writer structure"]
impl crate::Writable for SYSTCKCAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSTCKCAL to value 0x04"]
impl crate::Resettable for SYSTCKCAL_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
