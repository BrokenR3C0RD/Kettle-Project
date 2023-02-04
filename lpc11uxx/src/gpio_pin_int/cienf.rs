#[doc = "Register `CIENF` writer"]
pub struct W(crate::W<CIENF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIENF_SPEC>;
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
impl From<crate::W<CIENF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIENF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CENAF0` writer - Ones written to this address clears bits in the IENF, thus disabling interrupts. Bit n clears bit n in the IENF register. 0 = No operation. 1 = LOW-active interrupt selected or falling edge interrupt disabled."]
pub type CENAF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIENF_SPEC, bool, O>;
#[doc = "Field `CENAF1` writer - Ones written to this address clears bits in the IENF, thus disabling interrupts. Bit n clears bit n in the IENF register. 0 = No operation. 1 = LOW-active interrupt selected or falling edge interrupt disabled."]
pub type CENAF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIENF_SPEC, bool, O>;
#[doc = "Field `CENAF2` writer - Ones written to this address clears bits in the IENF, thus disabling interrupts. Bit n clears bit n in the IENF register. 0 = No operation. 1 = LOW-active interrupt selected or falling edge interrupt disabled."]
pub type CENAF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIENF_SPEC, bool, O>;
#[doc = "Field `CENAF3` writer - Ones written to this address clears bits in the IENF, thus disabling interrupts. Bit n clears bit n in the IENF register. 0 = No operation. 1 = LOW-active interrupt selected or falling edge interrupt disabled."]
pub type CENAF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIENF_SPEC, bool, O>;
#[doc = "Field `CENAF4` writer - Ones written to this address clears bits in the IENF, thus disabling interrupts. Bit n clears bit n in the IENF register. 0 = No operation. 1 = LOW-active interrupt selected or falling edge interrupt disabled."]
pub type CENAF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIENF_SPEC, bool, O>;
#[doc = "Field `CENAF5` writer - Ones written to this address clears bits in the IENF, thus disabling interrupts. Bit n clears bit n in the IENF register. 0 = No operation. 1 = LOW-active interrupt selected or falling edge interrupt disabled."]
pub type CENAF5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIENF_SPEC, bool, O>;
#[doc = "Field `CENAF6` writer - Ones written to this address clears bits in the IENF, thus disabling interrupts. Bit n clears bit n in the IENF register. 0 = No operation. 1 = LOW-active interrupt selected or falling edge interrupt disabled."]
pub type CENAF6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIENF_SPEC, bool, O>;
#[doc = "Field `CENAF7` writer - Ones written to this address clears bits in the IENF, thus disabling interrupts. Bit n clears bit n in the IENF register. 0 = No operation. 1 = LOW-active interrupt selected or falling edge interrupt disabled."]
pub type CENAF7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIENF_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Ones written to this address clears bits in the IENF, thus disabling interrupts. Bit n clears bit n in the IENF register. 0 = No operation. 1 = LOW-active interrupt selected or falling edge interrupt disabled."]
    #[inline(always)]
    #[must_use]
    pub fn cenaf0(&mut self) -> CENAF0_W<0> {
        CENAF0_W::new(self)
    }
    #[doc = "Bit 1 - Ones written to this address clears bits in the IENF, thus disabling interrupts. Bit n clears bit n in the IENF register. 0 = No operation. 1 = LOW-active interrupt selected or falling edge interrupt disabled."]
    #[inline(always)]
    #[must_use]
    pub fn cenaf1(&mut self) -> CENAF1_W<1> {
        CENAF1_W::new(self)
    }
    #[doc = "Bit 2 - Ones written to this address clears bits in the IENF, thus disabling interrupts. Bit n clears bit n in the IENF register. 0 = No operation. 1 = LOW-active interrupt selected or falling edge interrupt disabled."]
    #[inline(always)]
    #[must_use]
    pub fn cenaf2(&mut self) -> CENAF2_W<2> {
        CENAF2_W::new(self)
    }
    #[doc = "Bit 3 - Ones written to this address clears bits in the IENF, thus disabling interrupts. Bit n clears bit n in the IENF register. 0 = No operation. 1 = LOW-active interrupt selected or falling edge interrupt disabled."]
    #[inline(always)]
    #[must_use]
    pub fn cenaf3(&mut self) -> CENAF3_W<3> {
        CENAF3_W::new(self)
    }
    #[doc = "Bit 4 - Ones written to this address clears bits in the IENF, thus disabling interrupts. Bit n clears bit n in the IENF register. 0 = No operation. 1 = LOW-active interrupt selected or falling edge interrupt disabled."]
    #[inline(always)]
    #[must_use]
    pub fn cenaf4(&mut self) -> CENAF4_W<4> {
        CENAF4_W::new(self)
    }
    #[doc = "Bit 5 - Ones written to this address clears bits in the IENF, thus disabling interrupts. Bit n clears bit n in the IENF register. 0 = No operation. 1 = LOW-active interrupt selected or falling edge interrupt disabled."]
    #[inline(always)]
    #[must_use]
    pub fn cenaf5(&mut self) -> CENAF5_W<5> {
        CENAF5_W::new(self)
    }
    #[doc = "Bit 6 - Ones written to this address clears bits in the IENF, thus disabling interrupts. Bit n clears bit n in the IENF register. 0 = No operation. 1 = LOW-active interrupt selected or falling edge interrupt disabled."]
    #[inline(always)]
    #[must_use]
    pub fn cenaf6(&mut self) -> CENAF6_W<6> {
        CENAF6_W::new(self)
    }
    #[doc = "Bit 7 - Ones written to this address clears bits in the IENF, thus disabling interrupts. Bit n clears bit n in the IENF register. 0 = No operation. 1 = LOW-active interrupt selected or falling edge interrupt disabled."]
    #[inline(always)]
    #[must_use]
    pub fn cenaf7(&mut self) -> CENAF7_W<7> {
        CENAF7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear Pin Interrupt Enable Falling Edge / Active Level address\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cienf](index.html) module"]
pub struct CIENF_SPEC;
impl crate::RegisterSpec for CIENF_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cienf::W](W) writer structure"]
impl crate::Writable for CIENF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CIENF to value 0"]
impl crate::Resettable for CIENF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
