#[doc = "Register `FMSTATCLR` writer"]
pub struct W(crate::W<FMSTATCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMSTATCLR_SPEC>;
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
impl From<crate::W<FMSTATCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMSTATCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIG_DONE_CLR` writer - Writing a 1 to this bits clears the signature generation completion flag (SIG_DONE) in the FMSTAT register."]
pub type SIG_DONE_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMSTATCLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 2 - Writing a 1 to this bits clears the signature generation completion flag (SIG_DONE) in the FMSTAT register."]
    #[inline(always)]
    #[must_use]
    pub fn sig_done_clr(&mut self) -> SIG_DONE_CLR_W<2> {
        SIG_DONE_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Signature generation status clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmstatclr](index.html) module"]
pub struct FMSTATCLR_SPEC;
impl crate::RegisterSpec for FMSTATCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [fmstatclr::W](W) writer structure"]
impl crate::Writable for FMSTATCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMSTATCLR to value 0"]
impl crate::Resettable for FMSTATCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
