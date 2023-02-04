#[doc = "Register `FMSSTART` reader"]
pub struct R(crate::R<FMSSTART_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMSSTART_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMSSTART_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMSSTART_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMSSTART` writer"]
pub struct W(crate::W<FMSSTART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMSSTART_SPEC>;
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
impl From<crate::W<FMSSTART_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMSSTART_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - Signature generation start address (corresponds to AHB byte address bits\\[20:4\\])."]
pub type START_R = crate::FieldReader<u32, u32>;
#[doc = "Field `START` writer - Signature generation start address (corresponds to AHB byte address bits\\[20:4\\])."]
pub type START_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMSSTART_SPEC, u32, u32, 17, O>;
impl R {
    #[doc = "Bits 0:16 - Signature generation start address (corresponds to AHB byte address bits\\[20:4\\])."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - Signature generation start address (corresponds to AHB byte address bits\\[20:4\\])."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Signature start address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmsstart](index.html) module"]
pub struct FMSSTART_SPEC;
impl crate::RegisterSpec for FMSSTART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmsstart::R](R) reader structure"]
impl crate::Readable for FMSSTART_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmsstart::W](W) writer structure"]
impl crate::Writable for FMSSTART_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMSSTART to value 0"]
impl crate::Resettable for FMSSTART_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
