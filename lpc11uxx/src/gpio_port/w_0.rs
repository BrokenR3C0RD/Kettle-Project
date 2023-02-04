#[doc = "Register `W_0%s` reader"]
pub struct R(crate::R<W_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<W_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<W_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<W_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `W_0%s` writer"]
pub struct W(crate::W<W_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<W_0_SPEC>;
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
impl From<crate::W<W_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<W_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWORD` reader - Read 0: pin is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit."]
pub type PWORD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PWORD` writer - Read 0: pin is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit."]
pub type PWORD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, W_0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Read 0: pin is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit."]
    #[inline(always)]
    pub fn pword(&self) -> PWORD_R {
        PWORD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Read 0: pin is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit."]
    #[inline(always)]
    #[must_use]
    pub fn pword(&mut self) -> PWORD_W<0> {
        PWORD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Word pin registers port 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w_0](index.html) module"]
pub struct W_0_SPEC;
impl crate::RegisterSpec for W_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [w_0::R](R) reader structure"]
impl crate::Readable for W_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [w_0::W](W) writer structure"]
impl crate::Writable for W_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets W_0%s to value 0"]
impl crate::Resettable for W_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
