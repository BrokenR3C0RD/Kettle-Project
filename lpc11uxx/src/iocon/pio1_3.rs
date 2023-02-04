#[doc = "Register `PIO1_3` reader"]
pub struct R(crate::R<PIO1_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIO1_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIO1_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIO1_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIO1_3` writer"]
pub struct W(crate::W<PIO1_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIO1_3_SPEC>;
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
impl From<crate::W<PIO1_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIO1_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FUNC` reader - Selects pin function. Values 0x2 to 0x7 are reserved."]
pub type FUNC_R = crate::FieldReader<u8, FUNC_A>;
#[doc = "Selects pin function. Values 0x2 to 0x7 are reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FUNC_A {
    #[doc = "0: PIO1_3."]
    PIO1_3 = 0,
    #[doc = "1: CT32B1_MAT3."]
    CT32B1_MAT3 = 1,
}
impl From<FUNC_A> for u8 {
    #[inline(always)]
    fn from(variant: FUNC_A) -> Self {
        variant as _
    }
}
impl FUNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FUNC_A> {
        match self.bits {
            0 => Some(FUNC_A::PIO1_3),
            1 => Some(FUNC_A::CT32B1_MAT3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PIO1_3`"]
    #[inline(always)]
    pub fn is_pio1_3(&self) -> bool {
        *self == FUNC_A::PIO1_3
    }
    #[doc = "Checks if the value of the field is `CT32B1_MAT3`"]
    #[inline(always)]
    pub fn is_ct32b1_mat3(&self) -> bool {
        *self == FUNC_A::CT32B1_MAT3
    }
}
#[doc = "Field `FUNC` writer - Selects pin function. Values 0x2 to 0x7 are reserved."]
pub type FUNC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PIO1_3_SPEC, u8, FUNC_A, 3, O>;
impl<'a, const O: u8> FUNC_W<'a, O> {
    #[doc = "PIO1_3."]
    #[inline(always)]
    pub fn pio1_3(self) -> &'a mut W {
        self.variant(FUNC_A::PIO1_3)
    }
    #[doc = "CT32B1_MAT3."]
    #[inline(always)]
    pub fn ct32b1_mat3(self) -> &'a mut W {
        self.variant(FUNC_A::CT32B1_MAT3)
    }
}
#[doc = "Field `MODE` reader - Selects function mode (on-chip pull-up/pull-down resistor control)."]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "Selects function mode (on-chip pull-up/pull-down resistor control).\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Inactive (no pull-down/pull-up resistor enabled)."]
    FLOATING = 0,
    #[doc = "1: Pull-down resistor enabled."]
    PULL_DOWN = 1,
    #[doc = "2: Pull-up resistor enabled."]
    PULL_UP = 2,
    #[doc = "3: Repeater mode."]
    REPEATER = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::FLOATING,
            1 => MODE_A::PULL_DOWN,
            2 => MODE_A::PULL_UP,
            3 => MODE_A::REPEATER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLOATING`"]
    #[inline(always)]
    pub fn is_floating(&self) -> bool {
        *self == MODE_A::FLOATING
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == MODE_A::PULL_DOWN
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == MODE_A::REPEATER
    }
}
#[doc = "Field `MODE` writer - Selects function mode (on-chip pull-up/pull-down resistor control)."]
pub type MODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PIO1_3_SPEC, u8, MODE_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Inactive (no pull-down/pull-up resistor enabled)."]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(MODE_A::FLOATING)
    }
    #[doc = "Pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(MODE_A::PULL_DOWN)
    }
    #[doc = "Pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(MODE_A::PULL_UP)
    }
    #[doc = "Repeater mode."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(MODE_A::REPEATER)
    }
}
#[doc = "Field `HYS` reader - Hysteresis."]
pub type HYS_R = crate::BitReader<HYS_A>;
#[doc = "Hysteresis.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HYS_A {
    #[doc = "0: Disable."]
    DISABLED = 0,
    #[doc = "1: Enable."]
    ENABLED = 1,
}
impl From<HYS_A> for bool {
    #[inline(always)]
    fn from(variant: HYS_A) -> Self {
        variant as u8 != 0
    }
}
impl HYS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYS_A {
        match self.bits {
            false => HYS_A::DISABLED,
            true => HYS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HYS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HYS_A::ENABLED
    }
}
#[doc = "Field `HYS` writer - Hysteresis."]
pub type HYS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIO1_3_SPEC, HYS_A, O>;
impl<'a, const O: u8> HYS_W<'a, O> {
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HYS_A::DISABLED)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HYS_A::ENABLED)
    }
}
#[doc = "Field `INV` reader - Invert input"]
pub type INV_R = crate::BitReader<INV_A>;
#[doc = "Invert input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INV_A {
    #[doc = "0: Input not inverted (HIGH on pin reads as 1, LOW on pin reads as 0)."]
    NOT_INVERTED = 0,
    #[doc = "1: Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    INVERTED = 1,
}
impl From<INV_A> for bool {
    #[inline(always)]
    fn from(variant: INV_A) -> Self {
        variant as u8 != 0
    }
}
impl INV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INV_A {
        match self.bits {
            false => INV_A::NOT_INVERTED,
            true => INV_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_INVERTED`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == INV_A::NOT_INVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == INV_A::INVERTED
    }
}
#[doc = "Field `INV` writer - Invert input"]
pub type INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIO1_3_SPEC, INV_A, O>;
impl<'a, const O: u8> INV_W<'a, O> {
    #[doc = "Input not inverted (HIGH on pin reads as 1, LOW on pin reads as 0)."]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(INV_A::NOT_INVERTED)
    }
    #[doc = "Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(INV_A::INVERTED)
    }
}
#[doc = "Field `OD` reader - Open-drain mode."]
pub type OD_R = crate::BitReader<OD_A>;
#[doc = "Open-drain mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OD_A {
    #[doc = "0: Disable."]
    DISABLED = 0,
    #[doc = "1: Open-drain mode enabled. This is not a true open-drain mode. Input cannot be pulled up above VDD."]
    OPEN_DRAIN = 1,
}
impl From<OD_A> for bool {
    #[inline(always)]
    fn from(variant: OD_A) -> Self {
        variant as u8 != 0
    }
}
impl OD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OD_A {
        match self.bits {
            false => OD_A::DISABLED,
            true => OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == OD_A::OPEN_DRAIN
    }
}
#[doc = "Field `OD` writer - Open-drain mode."]
pub type OD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIO1_3_SPEC, OD_A, O>;
impl<'a, const O: u8> OD_W<'a, O> {
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OD_A::DISABLED)
    }
    #[doc = "Open-drain mode enabled. This is not a true open-drain mode. Input cannot be pulled up above VDD."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(OD_A::OPEN_DRAIN)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function. Values 0x2 to 0x7 are reserved."]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Hysteresis."]
    #[inline(always)]
    pub fn hys(&self) -> HYS_R {
        HYS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Invert input"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - Open-drain mode."]
    #[inline(always)]
    pub fn od(&self) -> OD_R {
        OD_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function. Values 0x2 to 0x7 are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn func(&mut self) -> FUNC_W<0> {
        FUNC_W::new(self)
    }
    #[doc = "Bits 3:4 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<3> {
        MODE_W::new(self)
    }
    #[doc = "Bit 5 - Hysteresis."]
    #[inline(always)]
    #[must_use]
    pub fn hys(&mut self) -> HYS_W<5> {
        HYS_W::new(self)
    }
    #[doc = "Bit 6 - Invert input"]
    #[inline(always)]
    #[must_use]
    pub fn inv(&mut self) -> INV_W<6> {
        INV_W::new(self)
    }
    #[doc = "Bit 10 - Open-drain mode."]
    #[inline(always)]
    #[must_use]
    pub fn od(&mut self) -> OD_W<10> {
        OD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O configuration for pin PIO1_3/CT32B1_MAT3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_3](index.html) module"]
pub struct PIO1_3_SPEC;
impl crate::RegisterSpec for PIO1_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pio1_3::R](R) reader structure"]
impl crate::Readable for PIO1_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pio1_3::W](W) writer structure"]
impl crate::Writable for PIO1_3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIO1_3 to value 0x90"]
impl crate::Resettable for PIO1_3_SPEC {
    const RESET_VALUE: Self::Ux = 0x90;
}
