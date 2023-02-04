#[doc = "Register `TV` reader"]
pub struct R(crate::R<TV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT` reader - Counter timer value."]
pub type COUNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - Counter timer value."]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Watchdog timer value register. This 24-bit register reads out the current value of the Watchdog timer.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tv](index.html) module"]
pub struct TV_SPEC;
impl crate::RegisterSpec for TV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tv::R](R) reader structure"]
impl crate::Readable for TV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TV to value 0xff"]
impl crate::Resettable for TV_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
