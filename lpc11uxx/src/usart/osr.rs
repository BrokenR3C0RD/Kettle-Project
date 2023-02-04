#[doc = "Register `OSR` reader"]
pub struct R(crate::R<OSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSR` writer"]
pub struct W(crate::W<OSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSR_SPEC>;
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
impl From<crate::W<OSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSFRAC` reader - Fractional part of the oversampling ratio, in units of 1/8th of an input clock period. (001 = 0.125, ..., 111 = 0.875)"]
pub type OSFRAC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OSFRAC` writer - Fractional part of the oversampling ratio, in units of 1/8th of an input clock period. (001 = 0.125, ..., 111 = 0.875)"]
pub type OSFRAC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSR_SPEC, u8, u8, 3, O>;
#[doc = "Field `OSINT` reader - Integer part of the oversampling ratio, minus 1. The reset values equate to the normal operating mode of 16 input clocks per bit time."]
pub type OSINT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OSINT` writer - Integer part of the oversampling ratio, minus 1. The reset values equate to the normal operating mode of 16 input clocks per bit time."]
pub type OSINT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSR_SPEC, u8, u8, 4, O>;
#[doc = "Field `FDINT` reader - In Smart Card mode, these bits act as a more-significant extension of the OSint field, allowing an oversampling ratio up to 2048 as required by ISO7816-3. In Smart Card mode, bits 14:4 should initially be set to 371, yielding an oversampling ratio of 372."]
pub type FDINT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FDINT` writer - In Smart Card mode, these bits act as a more-significant extension of the OSint field, allowing an oversampling ratio up to 2048 as required by ISO7816-3. In Smart Card mode, bits 14:4 should initially be set to 371, yielding an oversampling ratio of 372."]
pub type FDINT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSR_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 1:3 - Fractional part of the oversampling ratio, in units of 1/8th of an input clock period. (001 = 0.125, ..., 111 = 0.875)"]
    #[inline(always)]
    pub fn osfrac(&self) -> OSFRAC_R {
        OSFRAC_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:7 - Integer part of the oversampling ratio, minus 1. The reset values equate to the normal operating mode of 16 input clocks per bit time."]
    #[inline(always)]
    pub fn osint(&self) -> OSINT_R {
        OSINT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:14 - In Smart Card mode, these bits act as a more-significant extension of the OSint field, allowing an oversampling ratio up to 2048 as required by ISO7816-3. In Smart Card mode, bits 14:4 should initially be set to 371, yielding an oversampling ratio of 372."]
    #[inline(always)]
    pub fn fdint(&self) -> FDINT_R {
        FDINT_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:3 - Fractional part of the oversampling ratio, in units of 1/8th of an input clock period. (001 = 0.125, ..., 111 = 0.875)"]
    #[inline(always)]
    #[must_use]
    pub fn osfrac(&mut self) -> OSFRAC_W<1> {
        OSFRAC_W::new(self)
    }
    #[doc = "Bits 4:7 - Integer part of the oversampling ratio, minus 1. The reset values equate to the normal operating mode of 16 input clocks per bit time."]
    #[inline(always)]
    #[must_use]
    pub fn osint(&mut self) -> OSINT_W<4> {
        OSINT_W::new(self)
    }
    #[doc = "Bits 8:14 - In Smart Card mode, these bits act as a more-significant extension of the OSint field, allowing an oversampling ratio up to 2048 as required by ISO7816-3. In Smart Card mode, bits 14:4 should initially be set to 371, yielding an oversampling ratio of 372."]
    #[inline(always)]
    #[must_use]
    pub fn fdint(&mut self) -> FDINT_W<8> {
        FDINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Oversampling Register. Controls the degree of oversampling during each bit time.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osr](index.html) module"]
pub struct OSR_SPEC;
impl crate::RegisterSpec for OSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osr::R](R) reader structure"]
impl crate::Readable for OSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osr::W](W) writer structure"]
impl crate::Writable for OSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSR to value 0xf0"]
impl crate::Resettable for OSR_SPEC {
    const RESET_VALUE: Self::Ux = 0xf0;
}
