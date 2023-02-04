#[doc = "Register `PRESETCTRL` reader"]
pub struct R(crate::R<PRESETCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRESETCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRESETCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRESETCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRESETCTRL` writer"]
pub struct W(crate::W<PRESETCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRESETCTRL_SPEC>;
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
impl From<crate::W<PRESETCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRESETCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSP0` reader - SSP0 reset control"]
pub type SSP0_R = crate::BitReader<SSP0_A>;
#[doc = "SSP0 reset control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSP0_A {
    #[doc = "0: Resets the SSP0 peripheral."]
    ASSERTED = 0,
    #[doc = "1: SSP0 reset de-asserted."]
    DEASSERTED = 1,
}
impl From<SSP0_A> for bool {
    #[inline(always)]
    fn from(variant: SSP0_A) -> Self {
        variant as u8 != 0
    }
}
impl SSP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSP0_A {
        match self.bits {
            false => SSP0_A::ASSERTED,
            true => SSP0_A::DEASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == SSP0_A::ASSERTED
    }
    #[doc = "Checks if the value of the field is `DEASSERTED`"]
    #[inline(always)]
    pub fn is_deasserted(&self) -> bool {
        *self == SSP0_A::DEASSERTED
    }
}
#[doc = "Field `SSP0` writer - SSP0 reset control"]
pub type SSP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL_SPEC, SSP0_A, O>;
impl<'a, const O: u8> SSP0_W<'a, O> {
    #[doc = "Resets the SSP0 peripheral."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(SSP0_A::ASSERTED)
    }
    #[doc = "SSP0 reset de-asserted."]
    #[inline(always)]
    pub fn deasserted(self) -> &'a mut W {
        self.variant(SSP0_A::DEASSERTED)
    }
}
#[doc = "Field `I2C` reader - I2C reset control"]
pub type I2C_R = crate::BitReader<I2C_A>;
#[doc = "I2C reset control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_A {
    #[doc = "0: Resets the I2C peripheral."]
    ASSERTED = 0,
    #[doc = "1: I2C reset de-asserted."]
    DEASSERTED = 1,
}
impl From<I2C_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C_A {
        match self.bits {
            false => I2C_A::ASSERTED,
            true => I2C_A::DEASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == I2C_A::ASSERTED
    }
    #[doc = "Checks if the value of the field is `DEASSERTED`"]
    #[inline(always)]
    pub fn is_deasserted(&self) -> bool {
        *self == I2C_A::DEASSERTED
    }
}
#[doc = "Field `I2C` writer - I2C reset control"]
pub type I2C_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL_SPEC, I2C_A, O>;
impl<'a, const O: u8> I2C_W<'a, O> {
    #[doc = "Resets the I2C peripheral."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(I2C_A::ASSERTED)
    }
    #[doc = "I2C reset de-asserted."]
    #[inline(always)]
    pub fn deasserted(self) -> &'a mut W {
        self.variant(I2C_A::DEASSERTED)
    }
}
#[doc = "Field `SSP1` reader - SSP1 reset control"]
pub type SSP1_R = crate::BitReader<SSP1_A>;
#[doc = "SSP1 reset control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSP1_A {
    #[doc = "0: Resets the SSP1 peripheral."]
    ASSERTED = 0,
    #[doc = "1: SSP1 reset de-asserted."]
    DEASSERTED = 1,
}
impl From<SSP1_A> for bool {
    #[inline(always)]
    fn from(variant: SSP1_A) -> Self {
        variant as u8 != 0
    }
}
impl SSP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSP1_A {
        match self.bits {
            false => SSP1_A::ASSERTED,
            true => SSP1_A::DEASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == SSP1_A::ASSERTED
    }
    #[doc = "Checks if the value of the field is `DEASSERTED`"]
    #[inline(always)]
    pub fn is_deasserted(&self) -> bool {
        *self == SSP1_A::DEASSERTED
    }
}
#[doc = "Field `SSP1` writer - SSP1 reset control"]
pub type SSP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL_SPEC, SSP1_A, O>;
impl<'a, const O: u8> SSP1_W<'a, O> {
    #[doc = "Resets the SSP1 peripheral."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(SSP1_A::ASSERTED)
    }
    #[doc = "SSP1 reset de-asserted."]
    #[inline(always)]
    pub fn deasserted(self) -> &'a mut W {
        self.variant(SSP1_A::DEASSERTED)
    }
}
impl R {
    #[doc = "Bit 0 - SSP0 reset control"]
    #[inline(always)]
    pub fn ssp0(&self) -> SSP0_R {
        SSP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C reset control"]
    #[inline(always)]
    pub fn i2c(&self) -> I2C_R {
        I2C_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SSP1 reset control"]
    #[inline(always)]
    pub fn ssp1(&self) -> SSP1_R {
        SSP1_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSP0 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn ssp0(&mut self) -> SSP0_W<0> {
        SSP0_W::new(self)
    }
    #[doc = "Bit 1 - I2C reset control"]
    #[inline(always)]
    #[must_use]
    pub fn i2c(&mut self) -> I2C_W<1> {
        I2C_W::new(self)
    }
    #[doc = "Bit 2 - SSP1 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn ssp1(&mut self) -> SSP1_W<2> {
        SSP1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral reset control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [presetctrl](index.html) module"]
pub struct PRESETCTRL_SPEC;
impl crate::RegisterSpec for PRESETCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [presetctrl::R](R) reader structure"]
impl crate::Readable for PRESETCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [presetctrl::W](W) writer structure"]
impl crate::Writable for PRESETCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRESETCTRL to value 0"]
impl crate::Resettable for PRESETCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
