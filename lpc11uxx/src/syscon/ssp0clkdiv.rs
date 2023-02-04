#[doc = "Register `SSP0CLKDIV` reader"]
pub struct R(crate::R<SSP0CLKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSP0CLKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSP0CLKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSP0CLKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSP0CLKDIV` writer"]
pub struct W(crate::W<SSP0CLKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSP0CLKDIV_SPEC>;
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
impl From<crate::W<SSP0CLKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSP0CLKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV` reader - SPI0_PCLK clock divider values. 0: System clock disabled. 1: Divide by 1. to 255: Divide by 255."]
pub type DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIV` writer - SPI0_PCLK clock divider values. 0: System clock disabled. 1: Divide by 1. to 255: Divide by 255."]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SSP0CLKDIV_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - SPI0_PCLK clock divider values. 0: System clock disabled. 1: Divide by 1. to 255: Divide by 255."]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SPI0_PCLK clock divider values. 0: System clock disabled. 1: Divide by 1. to 255: Divide by 255."]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<0> {
        DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SSP0 clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssp0clkdiv](index.html) module"]
pub struct SSP0CLKDIV_SPEC;
impl crate::RegisterSpec for SSP0CLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssp0clkdiv::R](R) reader structure"]
impl crate::Readable for SSP0CLKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssp0clkdiv::W](W) writer structure"]
impl crate::Writable for SSP0CLKDIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSP0CLKDIV to value 0x01"]
impl crate::Resettable for SSP0CLKDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
