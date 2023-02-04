#[doc = "Register `CONCLR` writer"]
pub struct W(crate::W<CONCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONCLR_SPEC>;
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
impl From<crate::W<CONCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AAC` writer - Assert acknowledge Clear bit."]
pub type AAC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONCLR_SPEC, bool, O>;
#[doc = "Field `SIC` writer - I2C interrupt Clear bit."]
pub type SIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONCLR_SPEC, bool, O>;
#[doc = "Field `STAC` writer - START flag Clear bit."]
pub type STAC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONCLR_SPEC, bool, O>;
#[doc = "Field `I2ENC` writer - I2C interface Disable bit."]
pub type I2ENC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONCLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 2 - Assert acknowledge Clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn aac(&mut self) -> AAC_W<2> {
        AAC_W::new(self)
    }
    #[doc = "Bit 3 - I2C interrupt Clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn sic(&mut self) -> SIC_W<3> {
        SIC_W::new(self)
    }
    #[doc = "Bit 5 - START flag Clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn stac(&mut self) -> STAC_W<5> {
        STAC_W::new(self)
    }
    #[doc = "Bit 6 - I2C interface Disable bit."]
    #[inline(always)]
    #[must_use]
    pub fn i2enc(&mut self) -> I2ENC_W<6> {
        I2ENC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Control Clear Register. When a one is written to a bit of this register, the corresponding bit in the I2C control register is cleared. Writing a zero has no effect on the corresponding bit in the I2C control register.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conclr](index.html) module"]
pub struct CONCLR_SPEC;
impl crate::RegisterSpec for CONCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [conclr::W](W) writer structure"]
impl crate::Writable for CONCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONCLR to value 0"]
impl crate::Resettable for CONCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
