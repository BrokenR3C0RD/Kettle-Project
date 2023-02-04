#[doc = "Register `PCON` reader"]
pub struct R(crate::R<PCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCON` writer"]
pub struct W(crate::W<PCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCON_SPEC>;
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
impl From<crate::W<PCON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PM` reader - Power mode"]
pub type PM_R = crate::FieldReader<u8, PM_A>;
#[doc = "Power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PM_A {
    #[doc = "0: Default. The part is in active or sleep mode."]
    DEFAULT = 0,
    #[doc = "1: ARM WFI will enter Deep-sleep mode."]
    DEEPSLEEP = 1,
    #[doc = "2: ARM WFI will enter Power-down mode."]
    POWERDOWN = 2,
    #[doc = "3: ARM WFI will enter Deep-power down mode (ARM Cortex-M0 core powered-down)."]
    DEEPPOWERDOWN = 3,
}
impl From<PM_A> for u8 {
    #[inline(always)]
    fn from(variant: PM_A) -> Self {
        variant as _
    }
}
impl PM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PM_A> {
        match self.bits {
            0 => Some(PM_A::DEFAULT),
            1 => Some(PM_A::DEEPSLEEP),
            2 => Some(PM_A::POWERDOWN),
            3 => Some(PM_A::DEEPPOWERDOWN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == PM_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `DEEPSLEEP`"]
    #[inline(always)]
    pub fn is_deepsleep(&self) -> bool {
        *self == PM_A::DEEPSLEEP
    }
    #[doc = "Checks if the value of the field is `POWERDOWN`"]
    #[inline(always)]
    pub fn is_powerdown(&self) -> bool {
        *self == PM_A::POWERDOWN
    }
    #[doc = "Checks if the value of the field is `DEEPPOWERDOWN`"]
    #[inline(always)]
    pub fn is_deeppowerdown(&self) -> bool {
        *self == PM_A::DEEPPOWERDOWN
    }
}
#[doc = "Field `PM` writer - Power mode"]
pub type PM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCON_SPEC, u8, PM_A, 3, O>;
impl<'a, const O: u8> PM_W<'a, O> {
    #[doc = "Default. The part is in active or sleep mode."]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(PM_A::DEFAULT)
    }
    #[doc = "ARM WFI will enter Deep-sleep mode."]
    #[inline(always)]
    pub fn deepsleep(self) -> &'a mut W {
        self.variant(PM_A::DEEPSLEEP)
    }
    #[doc = "ARM WFI will enter Power-down mode."]
    #[inline(always)]
    pub fn powerdown(self) -> &'a mut W {
        self.variant(PM_A::POWERDOWN)
    }
    #[doc = "ARM WFI will enter Deep-power down mode (ARM Cortex-M0 core powered-down)."]
    #[inline(always)]
    pub fn deeppowerdown(self) -> &'a mut W {
        self.variant(PM_A::DEEPPOWERDOWN)
    }
}
#[doc = "Field `NODPD` reader - A 1 in this bit prevents entry to Deep power-down mode when 0x3 is written to the PM field above, the SLEEPDEEP bit is set, and a WFI is executed. Execution continues after the WFI if this bit is 1. This bit is cleared only by power-on reset, so writing a one to this bit locks the part in a mode in which Deep power-down mode is blocked."]
pub type NODPD_R = crate::BitReader<bool>;
#[doc = "Field `NODPD` writer - A 1 in this bit prevents entry to Deep power-down mode when 0x3 is written to the PM field above, the SLEEPDEEP bit is set, and a WFI is executed. Execution continues after the WFI if this bit is 1. This bit is cleared only by power-on reset, so writing a one to this bit locks the part in a mode in which Deep power-down mode is blocked."]
pub type NODPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCON_SPEC, bool, O>;
#[doc = "Field `SLEEPFLAG` reader - Sleep mode flag"]
pub type SLEEPFLAG_R = crate::BitReader<SLEEPFLAG_A>;
#[doc = "Sleep mode flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLEEPFLAG_A {
    #[doc = "0: Read: No power-down mode entered. LPC11U1x is in Active mode. Write: No effect."]
    NOPOWERDOWN = 0,
    #[doc = "1: Read: Sleep/Deep-sleep or Deep power-down mode entered. Write: Writing a 1 clears the SLEEPFLAG bit to 0."]
    POWERDOWN = 1,
}
impl From<SLEEPFLAG_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEPFLAG_A) -> Self {
        variant as u8 != 0
    }
}
impl SLEEPFLAG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEPFLAG_A {
        match self.bits {
            false => SLEEPFLAG_A::NOPOWERDOWN,
            true => SLEEPFLAG_A::POWERDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `NOPOWERDOWN`"]
    #[inline(always)]
    pub fn is_nopowerdown(&self) -> bool {
        *self == SLEEPFLAG_A::NOPOWERDOWN
    }
    #[doc = "Checks if the value of the field is `POWERDOWN`"]
    #[inline(always)]
    pub fn is_powerdown(&self) -> bool {
        *self == SLEEPFLAG_A::POWERDOWN
    }
}
#[doc = "Field `SLEEPFLAG` writer - Sleep mode flag"]
pub type SLEEPFLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCON_SPEC, SLEEPFLAG_A, O>;
impl<'a, const O: u8> SLEEPFLAG_W<'a, O> {
    #[doc = "Read: No power-down mode entered. LPC11U1x is in Active mode. Write: No effect."]
    #[inline(always)]
    pub fn nopowerdown(self) -> &'a mut W {
        self.variant(SLEEPFLAG_A::NOPOWERDOWN)
    }
    #[doc = "Read: Sleep/Deep-sleep or Deep power-down mode entered. Write: Writing a 1 clears the SLEEPFLAG bit to 0."]
    #[inline(always)]
    pub fn powerdown(self) -> &'a mut W {
        self.variant(SLEEPFLAG_A::POWERDOWN)
    }
}
#[doc = "Field `DPDFLAG` reader - Deep power-down flag"]
pub type DPDFLAG_R = crate::BitReader<DPDFLAG_A>;
#[doc = "Deep power-down flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPDFLAG_A {
    #[doc = "0: Read: Deep power-down mode not entered. Write: No effect."]
    DPNOTENTERED = 0,
    #[doc = "1: Read: Deep power-down mode entered. Write: Clear the Deep power-down flag."]
    DPENTERED = 1,
}
impl From<DPDFLAG_A> for bool {
    #[inline(always)]
    fn from(variant: DPDFLAG_A) -> Self {
        variant as u8 != 0
    }
}
impl DPDFLAG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPDFLAG_A {
        match self.bits {
            false => DPDFLAG_A::DPNOTENTERED,
            true => DPDFLAG_A::DPENTERED,
        }
    }
    #[doc = "Checks if the value of the field is `DPNOTENTERED`"]
    #[inline(always)]
    pub fn is_dpnotentered(&self) -> bool {
        *self == DPDFLAG_A::DPNOTENTERED
    }
    #[doc = "Checks if the value of the field is `DPENTERED`"]
    #[inline(always)]
    pub fn is_dpentered(&self) -> bool {
        *self == DPDFLAG_A::DPENTERED
    }
}
#[doc = "Field `DPDFLAG` writer - Deep power-down flag"]
pub type DPDFLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCON_SPEC, DPDFLAG_A, O>;
impl<'a, const O: u8> DPDFLAG_W<'a, O> {
    #[doc = "Read: Deep power-down mode not entered. Write: No effect."]
    #[inline(always)]
    pub fn dpnotentered(self) -> &'a mut W {
        self.variant(DPDFLAG_A::DPNOTENTERED)
    }
    #[doc = "Read: Deep power-down mode entered. Write: Clear the Deep power-down flag."]
    #[inline(always)]
    pub fn dpentered(self) -> &'a mut W {
        self.variant(DPDFLAG_A::DPENTERED)
    }
}
impl R {
    #[doc = "Bits 0:2 - Power mode"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - A 1 in this bit prevents entry to Deep power-down mode when 0x3 is written to the PM field above, the SLEEPDEEP bit is set, and a WFI is executed. Execution continues after the WFI if this bit is 1. This bit is cleared only by power-on reset, so writing a one to this bit locks the part in a mode in which Deep power-down mode is blocked."]
    #[inline(always)]
    pub fn nodpd(&self) -> NODPD_R {
        NODPD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Sleep mode flag"]
    #[inline(always)]
    pub fn sleepflag(&self) -> SLEEPFLAG_R {
        SLEEPFLAG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Deep power-down flag"]
    #[inline(always)]
    pub fn dpdflag(&self) -> DPDFLAG_R {
        DPDFLAG_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Power mode"]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<0> {
        PM_W::new(self)
    }
    #[doc = "Bit 3 - A 1 in this bit prevents entry to Deep power-down mode when 0x3 is written to the PM field above, the SLEEPDEEP bit is set, and a WFI is executed. Execution continues after the WFI if this bit is 1. This bit is cleared only by power-on reset, so writing a one to this bit locks the part in a mode in which Deep power-down mode is blocked."]
    #[inline(always)]
    #[must_use]
    pub fn nodpd(&mut self) -> NODPD_W<3> {
        NODPD_W::new(self)
    }
    #[doc = "Bit 8 - Sleep mode flag"]
    #[inline(always)]
    #[must_use]
    pub fn sleepflag(&mut self) -> SLEEPFLAG_W<8> {
        SLEEPFLAG_W::new(self)
    }
    #[doc = "Bit 11 - Deep power-down flag"]
    #[inline(always)]
    #[must_use]
    pub fn dpdflag(&mut self) -> DPDFLAG_W<11> {
        DPDFLAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcon](index.html) module"]
pub struct PCON_SPEC;
impl crate::RegisterSpec for PCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcon::R](R) reader structure"]
impl crate::Readable for PCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcon::W](W) writer structure"]
impl crate::Writable for PCON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCON to value 0"]
impl crate::Resettable for PCON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
