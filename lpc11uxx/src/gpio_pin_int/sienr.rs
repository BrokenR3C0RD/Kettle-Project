#[doc = "Register `SIENR` writer"]
pub struct W(crate::W<SIENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIENR_SPEC>;
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
impl From<crate::W<SIENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SETENRL0` writer - Ones written to this address set bits in the PINTEN_R, thus enabling interrupts. Bit n sets bit n in the PINTEN_R register. 0 = No operation. 1 = Enable rising edge or level interrupt."]
pub type SETENRL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIENR_SPEC, bool, O>;
#[doc = "Field `SETENRL1` writer - Ones written to this address set bits in the PINTEN_R, thus enabling interrupts. Bit n sets bit n in the PINTEN_R register. 0 = No operation. 1 = Enable rising edge or level interrupt."]
pub type SETENRL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIENR_SPEC, bool, O>;
#[doc = "Field `SETENRL2` writer - Ones written to this address set bits in the PINTEN_R, thus enabling interrupts. Bit n sets bit n in the PINTEN_R register. 0 = No operation. 1 = Enable rising edge or level interrupt."]
pub type SETENRL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIENR_SPEC, bool, O>;
#[doc = "Field `SETENRL3` writer - Ones written to this address set bits in the PINTEN_R, thus enabling interrupts. Bit n sets bit n in the PINTEN_R register. 0 = No operation. 1 = Enable rising edge or level interrupt."]
pub type SETENRL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIENR_SPEC, bool, O>;
#[doc = "Field `SETENRL4` writer - Ones written to this address set bits in the PINTEN_R, thus enabling interrupts. Bit n sets bit n in the PINTEN_R register. 0 = No operation. 1 = Enable rising edge or level interrupt."]
pub type SETENRL4_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIENR_SPEC, bool, O>;
#[doc = "Field `SETENRL5` writer - Ones written to this address set bits in the PINTEN_R, thus enabling interrupts. Bit n sets bit n in the PINTEN_R register. 0 = No operation. 1 = Enable rising edge or level interrupt."]
pub type SETENRL5_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIENR_SPEC, bool, O>;
#[doc = "Field `SETENRL6` writer - Ones written to this address set bits in the PINTEN_R, thus enabling interrupts. Bit n sets bit n in the PINTEN_R register. 0 = No operation. 1 = Enable rising edge or level interrupt."]
pub type SETENRL6_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIENR_SPEC, bool, O>;
#[doc = "Field `SETENRL7` writer - Ones written to this address set bits in the PINTEN_R, thus enabling interrupts. Bit n sets bit n in the PINTEN_R register. 0 = No operation. 1 = Enable rising edge or level interrupt."]
pub type SETENRL7_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIENR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Ones written to this address set bits in the PINTEN_R, thus enabling interrupts. Bit n sets bit n in the PINTEN_R register. 0 = No operation. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn setenrl0(&mut self) -> SETENRL0_W<0> {
        SETENRL0_W::new(self)
    }
    #[doc = "Bit 1 - Ones written to this address set bits in the PINTEN_R, thus enabling interrupts. Bit n sets bit n in the PINTEN_R register. 0 = No operation. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn setenrl1(&mut self) -> SETENRL1_W<1> {
        SETENRL1_W::new(self)
    }
    #[doc = "Bit 2 - Ones written to this address set bits in the PINTEN_R, thus enabling interrupts. Bit n sets bit n in the PINTEN_R register. 0 = No operation. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn setenrl2(&mut self) -> SETENRL2_W<2> {
        SETENRL2_W::new(self)
    }
    #[doc = "Bit 3 - Ones written to this address set bits in the PINTEN_R, thus enabling interrupts. Bit n sets bit n in the PINTEN_R register. 0 = No operation. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn setenrl3(&mut self) -> SETENRL3_W<3> {
        SETENRL3_W::new(self)
    }
    #[doc = "Bit 4 - Ones written to this address set bits in the PINTEN_R, thus enabling interrupts. Bit n sets bit n in the PINTEN_R register. 0 = No operation. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn setenrl4(&mut self) -> SETENRL4_W<4> {
        SETENRL4_W::new(self)
    }
    #[doc = "Bit 5 - Ones written to this address set bits in the PINTEN_R, thus enabling interrupts. Bit n sets bit n in the PINTEN_R register. 0 = No operation. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn setenrl5(&mut self) -> SETENRL5_W<5> {
        SETENRL5_W::new(self)
    }
    #[doc = "Bit 6 - Ones written to this address set bits in the PINTEN_R, thus enabling interrupts. Bit n sets bit n in the PINTEN_R register. 0 = No operation. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn setenrl6(&mut self) -> SETENRL6_W<6> {
        SETENRL6_W::new(self)
    }
    #[doc = "Bit 7 - Ones written to this address set bits in the PINTEN_R, thus enabling interrupts. Bit n sets bit n in the PINTEN_R register. 0 = No operation. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn setenrl7(&mut self) -> SETENRL7_W<7> {
        SETENRL7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set Pin Interrupt Enable (Rising) register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sienr](index.html) module"]
pub struct SIENR_SPEC;
impl crate::RegisterSpec for SIENR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sienr::W](W) writer structure"]
impl crate::Writable for SIENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIENR to value 0"]
impl crate::Resettable for SIENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
