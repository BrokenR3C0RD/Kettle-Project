#[doc = "Register `SIENF` writer"]
pub struct W(crate::W<SIENF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIENF_SPEC>;
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
impl From<crate::W<SIENF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIENF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SETENAF0` writer - Ones written to this address set bits in the IENF, thus enabling interrupts. Bit n sets bit n in the IENF register. 0 = No operation. 1 = Select HIGH-active interrupt or enable falling edge interrupt."]
pub type SETENAF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIENF_SPEC, bool, O>;
#[doc = "Field `SETENAF1` writer - Ones written to this address set bits in the IENF, thus enabling interrupts. Bit n sets bit n in the IENF register. 0 = No operation. 1 = Select HIGH-active interrupt or enable falling edge interrupt."]
pub type SETENAF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIENF_SPEC, bool, O>;
#[doc = "Field `SETENAF2` writer - Ones written to this address set bits in the IENF, thus enabling interrupts. Bit n sets bit n in the IENF register. 0 = No operation. 1 = Select HIGH-active interrupt or enable falling edge interrupt."]
pub type SETENAF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIENF_SPEC, bool, O>;
#[doc = "Field `SETENAF3` writer - Ones written to this address set bits in the IENF, thus enabling interrupts. Bit n sets bit n in the IENF register. 0 = No operation. 1 = Select HIGH-active interrupt or enable falling edge interrupt."]
pub type SETENAF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIENF_SPEC, bool, O>;
#[doc = "Field `SETENAF4` writer - Ones written to this address set bits in the IENF, thus enabling interrupts. Bit n sets bit n in the IENF register. 0 = No operation. 1 = Select HIGH-active interrupt or enable falling edge interrupt."]
pub type SETENAF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIENF_SPEC, bool, O>;
#[doc = "Field `SETENAF5` writer - Ones written to this address set bits in the IENF, thus enabling interrupts. Bit n sets bit n in the IENF register. 0 = No operation. 1 = Select HIGH-active interrupt or enable falling edge interrupt."]
pub type SETENAF5_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIENF_SPEC, bool, O>;
#[doc = "Field `SETENAF6` writer - Ones written to this address set bits in the IENF, thus enabling interrupts. Bit n sets bit n in the IENF register. 0 = No operation. 1 = Select HIGH-active interrupt or enable falling edge interrupt."]
pub type SETENAF6_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIENF_SPEC, bool, O>;
#[doc = "Field `SETENAF7` writer - Ones written to this address set bits in the IENF, thus enabling interrupts. Bit n sets bit n in the IENF register. 0 = No operation. 1 = Select HIGH-active interrupt or enable falling edge interrupt."]
pub type SETENAF7_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIENF_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Ones written to this address set bits in the IENF, thus enabling interrupts. Bit n sets bit n in the IENF register. 0 = No operation. 1 = Select HIGH-active interrupt or enable falling edge interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn setenaf0(&mut self) -> SETENAF0_W<0> {
        SETENAF0_W::new(self)
    }
    #[doc = "Bit 1 - Ones written to this address set bits in the IENF, thus enabling interrupts. Bit n sets bit n in the IENF register. 0 = No operation. 1 = Select HIGH-active interrupt or enable falling edge interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn setenaf1(&mut self) -> SETENAF1_W<1> {
        SETENAF1_W::new(self)
    }
    #[doc = "Bit 2 - Ones written to this address set bits in the IENF, thus enabling interrupts. Bit n sets bit n in the IENF register. 0 = No operation. 1 = Select HIGH-active interrupt or enable falling edge interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn setenaf2(&mut self) -> SETENAF2_W<2> {
        SETENAF2_W::new(self)
    }
    #[doc = "Bit 3 - Ones written to this address set bits in the IENF, thus enabling interrupts. Bit n sets bit n in the IENF register. 0 = No operation. 1 = Select HIGH-active interrupt or enable falling edge interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn setenaf3(&mut self) -> SETENAF3_W<3> {
        SETENAF3_W::new(self)
    }
    #[doc = "Bit 4 - Ones written to this address set bits in the IENF, thus enabling interrupts. Bit n sets bit n in the IENF register. 0 = No operation. 1 = Select HIGH-active interrupt or enable falling edge interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn setenaf4(&mut self) -> SETENAF4_W<4> {
        SETENAF4_W::new(self)
    }
    #[doc = "Bit 5 - Ones written to this address set bits in the IENF, thus enabling interrupts. Bit n sets bit n in the IENF register. 0 = No operation. 1 = Select HIGH-active interrupt or enable falling edge interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn setenaf5(&mut self) -> SETENAF5_W<5> {
        SETENAF5_W::new(self)
    }
    #[doc = "Bit 6 - Ones written to this address set bits in the IENF, thus enabling interrupts. Bit n sets bit n in the IENF register. 0 = No operation. 1 = Select HIGH-active interrupt or enable falling edge interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn setenaf6(&mut self) -> SETENAF6_W<6> {
        SETENAF6_W::new(self)
    }
    #[doc = "Bit 7 - Ones written to this address set bits in the IENF, thus enabling interrupts. Bit n sets bit n in the IENF register. 0 = No operation. 1 = Select HIGH-active interrupt or enable falling edge interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn setenaf7(&mut self) -> SETENAF7_W<7> {
        SETENAF7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set Pin Interrupt Enable Falling Edge / Active Level register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sienf](index.html) module"]
pub struct SIENF_SPEC;
impl crate::RegisterSpec for SIENF_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sienf::W](W) writer structure"]
impl crate::Writable for SIENF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIENF to value 0"]
impl crate::Resettable for SIENF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
