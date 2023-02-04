#[doc = "Register `STARTERP0` reader"]
pub struct R(crate::R<STARTERP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STARTERP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STARTERP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STARTERP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STARTERP0` writer"]
pub struct W(crate::W<STARTERP0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STARTERP0_SPEC>;
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
impl From<crate::W<STARTERP0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STARTERP0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PINT0` reader - Pin interrupt 0 wake-up"]
pub type PINT0_R = crate::BitReader<PINT0_A>;
#[doc = "Pin interrupt 0 wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINT0_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<PINT0_A> for bool {
    #[inline(always)]
    fn from(variant: PINT0_A) -> Self {
        variant as u8 != 0
    }
}
impl PINT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINT0_A {
        match self.bits {
            false => PINT0_A::DISABLED,
            true => PINT0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PINT0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PINT0_A::ENABLED
    }
}
#[doc = "Field `PINT0` writer - Pin interrupt 0 wake-up"]
pub type PINT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, PINT0_A, O>;
impl<'a, const O: u8> PINT0_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PINT0_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PINT0_A::ENABLED)
    }
}
#[doc = "Field `PINT1` reader - Pin interrupt 1 wake-up"]
pub type PINT1_R = crate::BitReader<PINT1_A>;
#[doc = "Pin interrupt 1 wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINT1_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<PINT1_A> for bool {
    #[inline(always)]
    fn from(variant: PINT1_A) -> Self {
        variant as u8 != 0
    }
}
impl PINT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINT1_A {
        match self.bits {
            false => PINT1_A::DISABLED,
            true => PINT1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PINT1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PINT1_A::ENABLED
    }
}
#[doc = "Field `PINT1` writer - Pin interrupt 1 wake-up"]
pub type PINT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, PINT1_A, O>;
impl<'a, const O: u8> PINT1_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PINT1_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PINT1_A::ENABLED)
    }
}
#[doc = "Field `PINT2` reader - Pin interrupt 2 wake-up"]
pub type PINT2_R = crate::BitReader<PINT2_A>;
#[doc = "Pin interrupt 2 wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINT2_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<PINT2_A> for bool {
    #[inline(always)]
    fn from(variant: PINT2_A) -> Self {
        variant as u8 != 0
    }
}
impl PINT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINT2_A {
        match self.bits {
            false => PINT2_A::DISABLED,
            true => PINT2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PINT2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PINT2_A::ENABLED
    }
}
#[doc = "Field `PINT2` writer - Pin interrupt 2 wake-up"]
pub type PINT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, PINT2_A, O>;
impl<'a, const O: u8> PINT2_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PINT2_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PINT2_A::ENABLED)
    }
}
#[doc = "Field `PINT3` reader - Pin interrupt 3 wake-up"]
pub type PINT3_R = crate::BitReader<PINT3_A>;
#[doc = "Pin interrupt 3 wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINT3_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<PINT3_A> for bool {
    #[inline(always)]
    fn from(variant: PINT3_A) -> Self {
        variant as u8 != 0
    }
}
impl PINT3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINT3_A {
        match self.bits {
            false => PINT3_A::DISABLED,
            true => PINT3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PINT3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PINT3_A::ENABLED
    }
}
#[doc = "Field `PINT3` writer - Pin interrupt 3 wake-up"]
pub type PINT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, PINT3_A, O>;
impl<'a, const O: u8> PINT3_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PINT3_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PINT3_A::ENABLED)
    }
}
#[doc = "Field `PINT4` reader - Pin interrupt 4 wake-up"]
pub type PINT4_R = crate::BitReader<PINT4_A>;
#[doc = "Pin interrupt 4 wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINT4_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<PINT4_A> for bool {
    #[inline(always)]
    fn from(variant: PINT4_A) -> Self {
        variant as u8 != 0
    }
}
impl PINT4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINT4_A {
        match self.bits {
            false => PINT4_A::DISABLED,
            true => PINT4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PINT4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PINT4_A::ENABLED
    }
}
#[doc = "Field `PINT4` writer - Pin interrupt 4 wake-up"]
pub type PINT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, PINT4_A, O>;
impl<'a, const O: u8> PINT4_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PINT4_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PINT4_A::ENABLED)
    }
}
#[doc = "Field `PINT5` reader - Pin interrupt 5 wake-up"]
pub type PINT5_R = crate::BitReader<PINT5_A>;
#[doc = "Pin interrupt 5 wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINT5_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<PINT5_A> for bool {
    #[inline(always)]
    fn from(variant: PINT5_A) -> Self {
        variant as u8 != 0
    }
}
impl PINT5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINT5_A {
        match self.bits {
            false => PINT5_A::DISABLED,
            true => PINT5_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PINT5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PINT5_A::ENABLED
    }
}
#[doc = "Field `PINT5` writer - Pin interrupt 5 wake-up"]
pub type PINT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, PINT5_A, O>;
impl<'a, const O: u8> PINT5_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PINT5_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PINT5_A::ENABLED)
    }
}
#[doc = "Field `PINT6` reader - Pin interrupt 6 wake-up"]
pub type PINT6_R = crate::BitReader<PINT6_A>;
#[doc = "Pin interrupt 6 wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINT6_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<PINT6_A> for bool {
    #[inline(always)]
    fn from(variant: PINT6_A) -> Self {
        variant as u8 != 0
    }
}
impl PINT6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINT6_A {
        match self.bits {
            false => PINT6_A::DISABLED,
            true => PINT6_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PINT6_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PINT6_A::ENABLED
    }
}
#[doc = "Field `PINT6` writer - Pin interrupt 6 wake-up"]
pub type PINT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, PINT6_A, O>;
impl<'a, const O: u8> PINT6_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PINT6_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PINT6_A::ENABLED)
    }
}
#[doc = "Field `PINT7` reader - Pin interrupt 7 wake-up"]
pub type PINT7_R = crate::BitReader<PINT7_A>;
#[doc = "Pin interrupt 7 wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINT7_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<PINT7_A> for bool {
    #[inline(always)]
    fn from(variant: PINT7_A) -> Self {
        variant as u8 != 0
    }
}
impl PINT7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINT7_A {
        match self.bits {
            false => PINT7_A::DISABLED,
            true => PINT7_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PINT7_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PINT7_A::ENABLED
    }
}
#[doc = "Field `PINT7` writer - Pin interrupt 7 wake-up"]
pub type PINT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, PINT7_A, O>;
impl<'a, const O: u8> PINT7_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PINT7_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PINT7_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Pin interrupt 0 wake-up"]
    #[inline(always)]
    pub fn pint0(&self) -> PINT0_R {
        PINT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin interrupt 1 wake-up"]
    #[inline(always)]
    pub fn pint1(&self) -> PINT1_R {
        PINT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin interrupt 2 wake-up"]
    #[inline(always)]
    pub fn pint2(&self) -> PINT2_R {
        PINT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin interrupt 3 wake-up"]
    #[inline(always)]
    pub fn pint3(&self) -> PINT3_R {
        PINT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin interrupt 4 wake-up"]
    #[inline(always)]
    pub fn pint4(&self) -> PINT4_R {
        PINT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin interrupt 5 wake-up"]
    #[inline(always)]
    pub fn pint5(&self) -> PINT5_R {
        PINT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin interrupt 6 wake-up"]
    #[inline(always)]
    pub fn pint6(&self) -> PINT6_R {
        PINT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin interrupt 7 wake-up"]
    #[inline(always)]
    pub fn pint7(&self) -> PINT7_R {
        PINT7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin interrupt 0 wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn pint0(&mut self) -> PINT0_W<0> {
        PINT0_W::new(self)
    }
    #[doc = "Bit 1 - Pin interrupt 1 wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn pint1(&mut self) -> PINT1_W<1> {
        PINT1_W::new(self)
    }
    #[doc = "Bit 2 - Pin interrupt 2 wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn pint2(&mut self) -> PINT2_W<2> {
        PINT2_W::new(self)
    }
    #[doc = "Bit 3 - Pin interrupt 3 wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn pint3(&mut self) -> PINT3_W<3> {
        PINT3_W::new(self)
    }
    #[doc = "Bit 4 - Pin interrupt 4 wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn pint4(&mut self) -> PINT4_W<4> {
        PINT4_W::new(self)
    }
    #[doc = "Bit 5 - Pin interrupt 5 wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn pint5(&mut self) -> PINT5_W<5> {
        PINT5_W::new(self)
    }
    #[doc = "Bit 6 - Pin interrupt 6 wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn pint6(&mut self) -> PINT6_W<6> {
        PINT6_W::new(self)
    }
    #[doc = "Bit 7 - Pin interrupt 7 wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn pint7(&mut self) -> PINT7_W<7> {
        PINT7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start logic 0 interrupt wake-up enable register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [starterp0](index.html) module"]
pub struct STARTERP0_SPEC;
impl crate::RegisterSpec for STARTERP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [starterp0::R](R) reader structure"]
impl crate::Readable for STARTERP0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [starterp0::W](W) writer structure"]
impl crate::Writable for STARTERP0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STARTERP0 to value 0"]
impl crate::Resettable for STARTERP0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
