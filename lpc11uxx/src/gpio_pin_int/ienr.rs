#[doc = "Register `IENR` reader"]
pub struct R(crate::R<IENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IENR` writer"]
pub struct W(crate::W<IENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IENR_SPEC>;
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
impl From<crate::W<IENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENRL0` reader - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
pub type ENRL0_R = crate::BitReader<bool>;
#[doc = "Field `ENRL0` writer - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
pub type ENRL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IENR_SPEC, bool, O>;
#[doc = "Field `ENRL1` reader - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
pub type ENRL1_R = crate::BitReader<bool>;
#[doc = "Field `ENRL1` writer - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
pub type ENRL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IENR_SPEC, bool, O>;
#[doc = "Field `ENRL2` reader - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
pub type ENRL2_R = crate::BitReader<bool>;
#[doc = "Field `ENRL2` writer - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
pub type ENRL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IENR_SPEC, bool, O>;
#[doc = "Field `ENRL3` reader - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
pub type ENRL3_R = crate::BitReader<bool>;
#[doc = "Field `ENRL3` writer - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
pub type ENRL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IENR_SPEC, bool, O>;
#[doc = "Field `ENRL4` reader - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
pub type ENRL4_R = crate::BitReader<bool>;
#[doc = "Field `ENRL4` writer - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
pub type ENRL4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IENR_SPEC, bool, O>;
#[doc = "Field `ENRL5` reader - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
pub type ENRL5_R = crate::BitReader<bool>;
#[doc = "Field `ENRL5` writer - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
pub type ENRL5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IENR_SPEC, bool, O>;
#[doc = "Field `ENRL6` reader - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
pub type ENRL6_R = crate::BitReader<bool>;
#[doc = "Field `ENRL6` writer - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
pub type ENRL6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IENR_SPEC, bool, O>;
#[doc = "Field `ENRL7` reader - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
pub type ENRL7_R = crate::BitReader<bool>;
#[doc = "Field `ENRL7` writer - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
pub type ENRL7_W<'a, const O: u8> = crate::BitWriter<'a, u32, IENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn enrl0(&self) -> ENRL0_R {
        ENRL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn enrl1(&self) -> ENRL1_R {
        ENRL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn enrl2(&self) -> ENRL2_R {
        ENRL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn enrl3(&self) -> ENRL3_R {
        ENRL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn enrl4(&self) -> ENRL4_R {
        ENRL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn enrl5(&self) -> ENRL5_R {
        ENRL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn enrl6(&self) -> ENRL6_R {
        ENRL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn enrl7(&self) -> ENRL7_R {
        ENRL7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn enrl0(&mut self) -> ENRL0_W<0> {
        ENRL0_W::new(self)
    }
    #[doc = "Bit 1 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn enrl1(&mut self) -> ENRL1_W<1> {
        ENRL1_W::new(self)
    }
    #[doc = "Bit 2 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn enrl2(&mut self) -> ENRL2_W<2> {
        ENRL2_W::new(self)
    }
    #[doc = "Bit 3 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn enrl3(&mut self) -> ENRL3_W<3> {
        ENRL3_W::new(self)
    }
    #[doc = "Bit 4 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn enrl4(&mut self) -> ENRL4_W<4> {
        ENRL4_W::new(self)
    }
    #[doc = "Bit 5 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn enrl5(&mut self) -> ENRL5_W<5> {
        ENRL5_W::new(self)
    }
    #[doc = "Bit 6 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn enrl6(&mut self) -> ENRL6_W<6> {
        ENRL6_W::new(self)
    }
    #[doc = "Bit 7 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn enrl7(&mut self) -> ENRL7_W<7> {
        ENRL7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin Interrupt Enable (Rising) register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ienr](index.html) module"]
pub struct IENR_SPEC;
impl crate::RegisterSpec for IENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ienr::R](R) reader structure"]
impl crate::Readable for IENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ienr::W](W) writer structure"]
impl crate::Writable for IENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IENR to value 0"]
impl crate::Resettable for IENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
