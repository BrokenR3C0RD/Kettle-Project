#[doc = "Register `SCLL` reader"]
pub struct R(crate::R<SCLL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCLL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCLL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCLL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCLL` writer"]
pub struct W(crate::W<SCLL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCLL_SPEC>;
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
impl From<crate::W<SCLL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCLL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCLL` reader - Count for SCL low time period selection."]
pub type SCLL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SCLL` writer - Count for SCL low time period selection."]
pub type SCLL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCLL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Count for SCL low time period selection."]
    #[inline(always)]
    pub fn scll(&self) -> SCLL_R {
        SCLL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Count for SCL low time period selection."]
    #[inline(always)]
    #[must_use]
    pub fn scll(&mut self) -> SCLL_W<0> {
        SCLL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCL Duty Cycle Register Low Half Word. Determines the low time of the I2C clock. I2nSCLL and I2nSCLH together determine the clock frequency generated by an I2C master and certain times used in slave mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scll](index.html) module"]
pub struct SCLL_SPEC;
impl crate::RegisterSpec for SCLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scll::R](R) reader structure"]
impl crate::Readable for SCLL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scll::W](W) writer structure"]
impl crate::Writable for SCLL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCLL to value 0x04"]
impl crate::Resettable for SCLL_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}