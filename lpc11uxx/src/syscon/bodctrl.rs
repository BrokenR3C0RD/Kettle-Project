#[doc = "Register `BODCTRL` reader"]
pub struct R(crate::R<BODCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BODCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BODCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BODCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BODCTRL` writer"]
pub struct W(crate::W<BODCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BODCTRL_SPEC>;
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
impl From<crate::W<BODCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BODCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BODRSTLEV` reader - BOD reset level"]
pub type BODRSTLEV_R = crate::FieldReader<u8, BODRSTLEV_A>;
#[doc = "BOD reset level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BODRSTLEV_A {
    #[doc = "0: Level 0: The reset assertion threshold voltage is 1.46 V; the reset de-assertion threshold voltage is 1.63 V."]
    LEVEL_0 = 0,
    #[doc = "1: Level 1: The reset assertion threshold voltage is 2.06 V; the reset de-assertion threshold voltage is 2.15 V."]
    LEVEL_1 = 1,
    #[doc = "2: Level 2: The reset assertion threshold voltage is 2.35 V; the reset de-assertion threshold voltage is 2.43 V."]
    LEVEL_2 = 2,
    #[doc = "3: Level 3: The reset assertion threshold voltage is 2.63 V; the reset de-assertion threshold voltage is 2.71 V."]
    LEVEL_3 = 3,
}
impl From<BODRSTLEV_A> for u8 {
    #[inline(always)]
    fn from(variant: BODRSTLEV_A) -> Self {
        variant as _
    }
}
impl BODRSTLEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODRSTLEV_A {
        match self.bits {
            0 => BODRSTLEV_A::LEVEL_0,
            1 => BODRSTLEV_A::LEVEL_1,
            2 => BODRSTLEV_A::LEVEL_2,
            3 => BODRSTLEV_A::LEVEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_0`"]
    #[inline(always)]
    pub fn is_level_0(&self) -> bool {
        *self == BODRSTLEV_A::LEVEL_0
    }
    #[doc = "Checks if the value of the field is `LEVEL_1`"]
    #[inline(always)]
    pub fn is_level_1(&self) -> bool {
        *self == BODRSTLEV_A::LEVEL_1
    }
    #[doc = "Checks if the value of the field is `LEVEL_2`"]
    #[inline(always)]
    pub fn is_level_2(&self) -> bool {
        *self == BODRSTLEV_A::LEVEL_2
    }
    #[doc = "Checks if the value of the field is `LEVEL_3`"]
    #[inline(always)]
    pub fn is_level_3(&self) -> bool {
        *self == BODRSTLEV_A::LEVEL_3
    }
}
#[doc = "Field `BODRSTLEV` writer - BOD reset level"]
pub type BODRSTLEV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, BODCTRL_SPEC, u8, BODRSTLEV_A, 2, O>;
impl<'a, const O: u8> BODRSTLEV_W<'a, O> {
    #[doc = "Level 0: The reset assertion threshold voltage is 1.46 V; the reset de-assertion threshold voltage is 1.63 V."]
    #[inline(always)]
    pub fn level_0(self) -> &'a mut W {
        self.variant(BODRSTLEV_A::LEVEL_0)
    }
    #[doc = "Level 1: The reset assertion threshold voltage is 2.06 V; the reset de-assertion threshold voltage is 2.15 V."]
    #[inline(always)]
    pub fn level_1(self) -> &'a mut W {
        self.variant(BODRSTLEV_A::LEVEL_1)
    }
    #[doc = "Level 2: The reset assertion threshold voltage is 2.35 V; the reset de-assertion threshold voltage is 2.43 V."]
    #[inline(always)]
    pub fn level_2(self) -> &'a mut W {
        self.variant(BODRSTLEV_A::LEVEL_2)
    }
    #[doc = "Level 3: The reset assertion threshold voltage is 2.63 V; the reset de-assertion threshold voltage is 2.71 V."]
    #[inline(always)]
    pub fn level_3(self) -> &'a mut W {
        self.variant(BODRSTLEV_A::LEVEL_3)
    }
}
#[doc = "Field `BODINTVAL` reader - BOD interrupt level"]
pub type BODINTVAL_R = crate::FieldReader<u8, BODINTVAL_A>;
#[doc = "BOD interrupt level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BODINTVAL_A {
    #[doc = "1: Level 1:The interrupt assertion threshold voltage is 2.22 V; the interrupt de-assertion threshold voltage is 2.35 V."]
    LEVEL_1 = 1,
    #[doc = "2: Level 2: The interrupt assertion threshold voltage is 2.52 V; the interrupt de-assertion threshold voltage is 2.66 V."]
    LEVEL_2 = 2,
    #[doc = "3: Level 3: The interrupt assertion threshold voltage is 2.80 V; the interrupt de-assertion threshold voltage is 2.90 V."]
    LEVEL_3 = 3,
}
impl From<BODINTVAL_A> for u8 {
    #[inline(always)]
    fn from(variant: BODINTVAL_A) -> Self {
        variant as _
    }
}
impl BODINTVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BODINTVAL_A> {
        match self.bits {
            1 => Some(BODINTVAL_A::LEVEL_1),
            2 => Some(BODINTVAL_A::LEVEL_2),
            3 => Some(BODINTVAL_A::LEVEL_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_1`"]
    #[inline(always)]
    pub fn is_level_1(&self) -> bool {
        *self == BODINTVAL_A::LEVEL_1
    }
    #[doc = "Checks if the value of the field is `LEVEL_2`"]
    #[inline(always)]
    pub fn is_level_2(&self) -> bool {
        *self == BODINTVAL_A::LEVEL_2
    }
    #[doc = "Checks if the value of the field is `LEVEL_3`"]
    #[inline(always)]
    pub fn is_level_3(&self) -> bool {
        *self == BODINTVAL_A::LEVEL_3
    }
}
#[doc = "Field `BODINTVAL` writer - BOD interrupt level"]
pub type BODINTVAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BODCTRL_SPEC, u8, BODINTVAL_A, 2, O>;
impl<'a, const O: u8> BODINTVAL_W<'a, O> {
    #[doc = "Level 1:The interrupt assertion threshold voltage is 2.22 V; the interrupt de-assertion threshold voltage is 2.35 V."]
    #[inline(always)]
    pub fn level_1(self) -> &'a mut W {
        self.variant(BODINTVAL_A::LEVEL_1)
    }
    #[doc = "Level 2: The interrupt assertion threshold voltage is 2.52 V; the interrupt de-assertion threshold voltage is 2.66 V."]
    #[inline(always)]
    pub fn level_2(self) -> &'a mut W {
        self.variant(BODINTVAL_A::LEVEL_2)
    }
    #[doc = "Level 3: The interrupt assertion threshold voltage is 2.80 V; the interrupt de-assertion threshold voltage is 2.90 V."]
    #[inline(always)]
    pub fn level_3(self) -> &'a mut W {
        self.variant(BODINTVAL_A::LEVEL_3)
    }
}
#[doc = "Field `BODRSTENA` reader - BOD reset enable"]
pub type BODRSTENA_R = crate::BitReader<BODRSTENA_A>;
#[doc = "BOD reset enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BODRSTENA_A {
    #[doc = "0: Disable reset function."]
    DISABLE = 0,
    #[doc = "1: Enable reset function."]
    ENABLE = 1,
}
impl From<BODRSTENA_A> for bool {
    #[inline(always)]
    fn from(variant: BODRSTENA_A) -> Self {
        variant as u8 != 0
    }
}
impl BODRSTENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODRSTENA_A {
        match self.bits {
            false => BODRSTENA_A::DISABLE,
            true => BODRSTENA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BODRSTENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BODRSTENA_A::ENABLE
    }
}
#[doc = "Field `BODRSTENA` writer - BOD reset enable"]
pub type BODRSTENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, BODCTRL_SPEC, BODRSTENA_A, O>;
impl<'a, const O: u8> BODRSTENA_W<'a, O> {
    #[doc = "Disable reset function."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BODRSTENA_A::DISABLE)
    }
    #[doc = "Enable reset function."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BODRSTENA_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:1 - BOD reset level"]
    #[inline(always)]
    pub fn bodrstlev(&self) -> BODRSTLEV_R {
        BODRSTLEV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - BOD interrupt level"]
    #[inline(always)]
    pub fn bodintval(&self) -> BODINTVAL_R {
        BODINTVAL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - BOD reset enable"]
    #[inline(always)]
    pub fn bodrstena(&self) -> BODRSTENA_R {
        BODRSTENA_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - BOD reset level"]
    #[inline(always)]
    #[must_use]
    pub fn bodrstlev(&mut self) -> BODRSTLEV_W<0> {
        BODRSTLEV_W::new(self)
    }
    #[doc = "Bits 2:3 - BOD interrupt level"]
    #[inline(always)]
    #[must_use]
    pub fn bodintval(&mut self) -> BODINTVAL_W<2> {
        BODINTVAL_W::new(self)
    }
    #[doc = "Bit 4 - BOD reset enable"]
    #[inline(always)]
    #[must_use]
    pub fn bodrstena(&mut self) -> BODRSTENA_W<4> {
        BODRSTENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Brown-Out Detect\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bodctrl](index.html) module"]
pub struct BODCTRL_SPEC;
impl crate::RegisterSpec for BODCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bodctrl::R](R) reader structure"]
impl crate::Readable for BODCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bodctrl::W](W) writer structure"]
impl crate::Writable for BODCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BODCTRL to value 0"]
impl crate::Resettable for BODCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
