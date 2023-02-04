#[doc = "Register `GPREG4` reader"]
pub struct R(crate::R<GPREG4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPREG4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPREG4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPREG4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPREG4` writer"]
pub struct W(crate::W<GPREG4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPREG4_SPEC>;
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
impl From<crate::W<GPREG4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPREG4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAKEUPHYS` reader - WAKEUP pin hysteresis enable"]
pub type WAKEUPHYS_R = crate::BitReader<WAKEUPHYS_A>;
#[doc = "WAKEUP pin hysteresis enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAKEUPHYS_A {
    #[doc = "0: Hysteresis for WAKEUP pin disabled."]
    DISABLED = 0,
    #[doc = "1: Hysteresis for WAKEUP pin enabled."]
    ENABLED = 1,
}
impl From<WAKEUPHYS_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEUPHYS_A) -> Self {
        variant as u8 != 0
    }
}
impl WAKEUPHYS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEUPHYS_A {
        match self.bits {
            false => WAKEUPHYS_A::DISABLED,
            true => WAKEUPHYS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAKEUPHYS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WAKEUPHYS_A::ENABLED
    }
}
#[doc = "Field `WAKEUPHYS` writer - WAKEUP pin hysteresis enable"]
pub type WAKEUPHYS_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPREG4_SPEC, WAKEUPHYS_A, O>;
impl<'a, const O: u8> WAKEUPHYS_W<'a, O> {
    #[doc = "Hysteresis for WAKEUP pin disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAKEUPHYS_A::DISABLED)
    }
    #[doc = "Hysteresis for WAKEUP pin enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAKEUPHYS_A::ENABLED)
    }
}
#[doc = "Field `GPDATA` reader - Data retained during Deep power-down mode."]
pub type GPDATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GPDATA` writer - Data retained during Deep power-down mode."]
pub type GPDATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPREG4_SPEC, u32, u32, 21, O>;
impl R {
    #[doc = "Bit 10 - WAKEUP pin hysteresis enable"]
    #[inline(always)]
    pub fn wakeuphys(&self) -> WAKEUPHYS_R {
        WAKEUPHYS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:31 - Data retained during Deep power-down mode."]
    #[inline(always)]
    pub fn gpdata(&self) -> GPDATA_R {
        GPDATA_R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bit 10 - WAKEUP pin hysteresis enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeuphys(&mut self) -> WAKEUPHYS_W<10> {
        WAKEUPHYS_W::new(self)
    }
    #[doc = "Bits 11:31 - Data retained during Deep power-down mode."]
    #[inline(always)]
    #[must_use]
    pub fn gpdata(&mut self) -> GPDATA_W<11> {
        GPDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General purpose register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpreg4](index.html) module"]
pub struct GPREG4_SPEC;
impl crate::RegisterSpec for GPREG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpreg4::R](R) reader structure"]
impl crate::Readable for GPREG4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpreg4::W](W) writer structure"]
impl crate::Writable for GPREG4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPREG4 to value 0"]
impl crate::Resettable for GPREG4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
