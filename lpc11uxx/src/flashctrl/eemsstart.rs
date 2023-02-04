#[doc = "Register `EEMSSTART` reader"]
pub struct R(crate::R<EEMSSTART_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEMSSTART_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEMSSTART_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEMSSTART_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EEMSSTART` writer"]
pub struct W(crate::W<EEMSSTART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEMSSTART_SPEC>;
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
impl From<crate::W<EEMSSTART_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEMSSTART_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STARTA` reader - BIST start address: Bit 0 is fixed zero since only even addresses are allowed."]
pub type STARTA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STARTA` writer - BIST start address: Bit 0 is fixed zero since only even addresses are allowed."]
pub type STARTA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EEMSSTART_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - BIST start address: Bit 0 is fixed zero since only even addresses are allowed."]
    #[inline(always)]
    pub fn starta(&self) -> STARTA_R {
        STARTA_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - BIST start address: Bit 0 is fixed zero since only even addresses are allowed."]
    #[inline(always)]
    #[must_use]
    pub fn starta(&mut self) -> STARTA_W<0> {
        STARTA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM BIST start address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eemsstart](index.html) module"]
pub struct EEMSSTART_SPEC;
impl crate::RegisterSpec for EEMSSTART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eemsstart::R](R) reader structure"]
impl crate::Readable for EEMSSTART_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eemsstart::W](W) writer structure"]
impl crate::Writable for EEMSSTART_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EEMSSTART to value 0"]
impl crate::Resettable for EEMSSTART_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
