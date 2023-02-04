#[doc = "Register `DLM` reader"]
pub struct R(crate::R<DLM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DLM` writer"]
pub struct W(crate::W<DLM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLM_SPEC>;
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
impl From<crate::W<DLM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLMSB` reader - The USART Divisor Latch MSB Register, along with the DLL register, determines the baud rate of the USART."]
pub type DLMSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLMSB` writer - The USART Divisor Latch MSB Register, along with the DLL register, determines the baud rate of the USART."]
pub type DLMSB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DLM_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - The USART Divisor Latch MSB Register, along with the DLL register, determines the baud rate of the USART."]
    #[inline(always)]
    pub fn dlmsb(&self) -> DLMSB_R {
        DLMSB_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The USART Divisor Latch MSB Register, along with the DLL register, determines the baud rate of the USART."]
    #[inline(always)]
    #[must_use]
    pub fn dlmsb(&mut self) -> DLMSB_W<0> {
        DLMSB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider. (DLAB=1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlm](index.html) module"]
pub struct DLM_SPEC;
impl crate::RegisterSpec for DLM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dlm::R](R) reader structure"]
impl crate::Readable for DLM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dlm::W](W) writer structure"]
impl crate::Writable for DLM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DLM to value 0"]
impl crate::Resettable for DLM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
