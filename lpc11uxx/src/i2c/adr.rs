#[doc = "Register `ADR%s` reader"]
pub struct R(crate::R<ADR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADR%s` writer"]
pub struct W(crate::W<ADR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADR_SPEC>;
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
impl From<crate::W<ADR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GC` reader - General Call enable bit."]
pub type GC_R = crate::BitReader<bool>;
#[doc = "Field `GC` writer - General Call enable bit."]
pub type GC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADR_SPEC, bool, O>;
#[doc = "Field `Address` reader - The I2C device address for slave mode."]
pub type ADDRESS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Address` writer - The I2C device address for slave mode."]
pub type ADDRESS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADR_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - General Call enable bit."]
    #[inline(always)]
    pub fn gc(&self) -> GC_R {
        GC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - The I2C device address for slave mode."]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - General Call enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn gc(&mut self) -> GC_W<0> {
        GC_W::new(self)
    }
    #[doc = "Bits 1:7 - The I2C device address for slave mode."]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> ADDRESS_W<1> {
        ADDRESS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Slave Address Register. Contains the 7-bit slave address for operation of the I2C interface in slave mode, and is not used in master mode. The least significant bit determines whether a slave responds to the General Call address.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adr](index.html) module"]
pub struct ADR_SPEC;
impl crate::RegisterSpec for ADR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adr::R](R) reader structure"]
impl crate::Readable for ADR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adr::W](W) writer structure"]
impl crate::Writable for ADR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADR%s to value 0"]
impl crate::Resettable for ADR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
