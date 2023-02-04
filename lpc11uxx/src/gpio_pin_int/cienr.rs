#[doc = "Register `CIENR` writer"]
pub struct W(crate::W<CIENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIENR_SPEC>;
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
impl From<crate::W<CIENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CENRL0` writer - Ones written to this address clear bits in the IENR, thus disabling the interrupts. Bit n clears bit n in the IENR register. 0 = No operation. 1 = Disable rising edge or level interrupt."]
pub type CENRL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIENR_SPEC, bool, O>;
#[doc = "Field `CENRL1` writer - Ones written to this address clear bits in the IENR, thus disabling the interrupts. Bit n clears bit n in the IENR register. 0 = No operation. 1 = Disable rising edge or level interrupt."]
pub type CENRL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIENR_SPEC, bool, O>;
#[doc = "Field `CENRL2` writer - Ones written to this address clear bits in the IENR, thus disabling the interrupts. Bit n clears bit n in the IENR register. 0 = No operation. 1 = Disable rising edge or level interrupt."]
pub type CENRL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIENR_SPEC, bool, O>;
#[doc = "Field `CENRL3` writer - Ones written to this address clear bits in the IENR, thus disabling the interrupts. Bit n clears bit n in the IENR register. 0 = No operation. 1 = Disable rising edge or level interrupt."]
pub type CENRL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIENR_SPEC, bool, O>;
#[doc = "Field `CENRL4` writer - Ones written to this address clear bits in the IENR, thus disabling the interrupts. Bit n clears bit n in the IENR register. 0 = No operation. 1 = Disable rising edge or level interrupt."]
pub type CENRL4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIENR_SPEC, bool, O>;
#[doc = "Field `CENRL5` writer - Ones written to this address clear bits in the IENR, thus disabling the interrupts. Bit n clears bit n in the IENR register. 0 = No operation. 1 = Disable rising edge or level interrupt."]
pub type CENRL5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIENR_SPEC, bool, O>;
#[doc = "Field `CENRL6` writer - Ones written to this address clear bits in the IENR, thus disabling the interrupts. Bit n clears bit n in the IENR register. 0 = No operation. 1 = Disable rising edge or level interrupt."]
pub type CENRL6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIENR_SPEC, bool, O>;
#[doc = "Field `CENRL7` writer - Ones written to this address clear bits in the IENR, thus disabling the interrupts. Bit n clears bit n in the IENR register. 0 = No operation. 1 = Disable rising edge or level interrupt."]
pub type CENRL7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIENR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Ones written to this address clear bits in the IENR, thus disabling the interrupts. Bit n clears bit n in the IENR register. 0 = No operation. 1 = Disable rising edge or level interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cenrl0(&mut self) -> CENRL0_W<0> {
        CENRL0_W::new(self)
    }
    #[doc = "Bit 1 - Ones written to this address clear bits in the IENR, thus disabling the interrupts. Bit n clears bit n in the IENR register. 0 = No operation. 1 = Disable rising edge or level interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cenrl1(&mut self) -> CENRL1_W<1> {
        CENRL1_W::new(self)
    }
    #[doc = "Bit 2 - Ones written to this address clear bits in the IENR, thus disabling the interrupts. Bit n clears bit n in the IENR register. 0 = No operation. 1 = Disable rising edge or level interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cenrl2(&mut self) -> CENRL2_W<2> {
        CENRL2_W::new(self)
    }
    #[doc = "Bit 3 - Ones written to this address clear bits in the IENR, thus disabling the interrupts. Bit n clears bit n in the IENR register. 0 = No operation. 1 = Disable rising edge or level interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cenrl3(&mut self) -> CENRL3_W<3> {
        CENRL3_W::new(self)
    }
    #[doc = "Bit 4 - Ones written to this address clear bits in the IENR, thus disabling the interrupts. Bit n clears bit n in the IENR register. 0 = No operation. 1 = Disable rising edge or level interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cenrl4(&mut self) -> CENRL4_W<4> {
        CENRL4_W::new(self)
    }
    #[doc = "Bit 5 - Ones written to this address clear bits in the IENR, thus disabling the interrupts. Bit n clears bit n in the IENR register. 0 = No operation. 1 = Disable rising edge or level interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cenrl5(&mut self) -> CENRL5_W<5> {
        CENRL5_W::new(self)
    }
    #[doc = "Bit 6 - Ones written to this address clear bits in the IENR, thus disabling the interrupts. Bit n clears bit n in the IENR register. 0 = No operation. 1 = Disable rising edge or level interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cenrl6(&mut self) -> CENRL6_W<6> {
        CENRL6_W::new(self)
    }
    #[doc = "Bit 7 - Ones written to this address clear bits in the IENR, thus disabling the interrupts. Bit n clears bit n in the IENR register. 0 = No operation. 1 = Disable rising edge or level interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cenrl7(&mut self) -> CENRL7_W<7> {
        CENRL7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear Pin Interrupt Enable (Rising) register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cienr](index.html) module"]
pub struct CIENR_SPEC;
impl crate::RegisterSpec for CIENR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cienr::W](W) writer structure"]
impl crate::Writable for CIENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CIENR to value 0"]
impl crate::Resettable for CIENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
