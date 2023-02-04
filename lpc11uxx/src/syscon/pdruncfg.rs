#[doc = "Register `PDRUNCFG` reader"]
pub struct R(crate::R<PDRUNCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDRUNCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDRUNCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDRUNCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDRUNCFG` writer"]
pub struct W(crate::W<PDRUNCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDRUNCFG_SPEC>;
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
impl From<crate::W<PDRUNCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDRUNCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRCOUT_PD` reader - IRC oscillator output power-down"]
pub type IRCOUT_PD_R = crate::BitReader<IRCOUT_PD_A>;
#[doc = "IRC oscillator output power-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRCOUT_PD_A {
    #[doc = "0: Powered"]
    POWERED = 0,
    #[doc = "1: Powered down"]
    POWERED_DOWN = 1,
}
impl From<IRCOUT_PD_A> for bool {
    #[inline(always)]
    fn from(variant: IRCOUT_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl IRCOUT_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRCOUT_PD_A {
        match self.bits {
            false => IRCOUT_PD_A::POWERED,
            true => IRCOUT_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == IRCOUT_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == IRCOUT_PD_A::POWERED_DOWN
    }
}
#[doc = "Field `IRCOUT_PD` writer - IRC oscillator output power-down"]
pub type IRCOUT_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG_SPEC, IRCOUT_PD_A, O>;
impl<'a, const O: u8> IRCOUT_PD_W<'a, O> {
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(IRCOUT_PD_A::POWERED)
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(IRCOUT_PD_A::POWERED_DOWN)
    }
}
#[doc = "Field `IRC_PD` reader - IRC oscillator power-down"]
pub type IRC_PD_R = crate::BitReader<IRC_PD_A>;
#[doc = "IRC oscillator power-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRC_PD_A {
    #[doc = "0: Powered"]
    POWERED = 0,
    #[doc = "1: Powered down"]
    POWERED_DOWN = 1,
}
impl From<IRC_PD_A> for bool {
    #[inline(always)]
    fn from(variant: IRC_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl IRC_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRC_PD_A {
        match self.bits {
            false => IRC_PD_A::POWERED,
            true => IRC_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == IRC_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == IRC_PD_A::POWERED_DOWN
    }
}
#[doc = "Field `IRC_PD` writer - IRC oscillator power-down"]
pub type IRC_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG_SPEC, IRC_PD_A, O>;
impl<'a, const O: u8> IRC_PD_W<'a, O> {
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(IRC_PD_A::POWERED)
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(IRC_PD_A::POWERED_DOWN)
    }
}
#[doc = "Field `FLASH_PD` reader - Flash power-down"]
pub type FLASH_PD_R = crate::BitReader<FLASH_PD_A>;
#[doc = "Flash power-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASH_PD_A {
    #[doc = "0: Powered"]
    POWERED = 0,
    #[doc = "1: Powered down"]
    POWERED_DOWN = 1,
}
impl From<FLASH_PD_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl FLASH_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH_PD_A {
        match self.bits {
            false => FLASH_PD_A::POWERED,
            true => FLASH_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == FLASH_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == FLASH_PD_A::POWERED_DOWN
    }
}
#[doc = "Field `FLASH_PD` writer - Flash power-down"]
pub type FLASH_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG_SPEC, FLASH_PD_A, O>;
impl<'a, const O: u8> FLASH_PD_W<'a, O> {
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(FLASH_PD_A::POWERED)
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(FLASH_PD_A::POWERED_DOWN)
    }
}
#[doc = "Field `BOD_PD` reader - BOD power-down"]
pub type BOD_PD_R = crate::BitReader<BOD_PD_A>;
#[doc = "BOD power-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOD_PD_A {
    #[doc = "0: Powered"]
    POWERED = 0,
    #[doc = "1: Powered down"]
    POWERED_DOWN = 1,
}
impl From<BOD_PD_A> for bool {
    #[inline(always)]
    fn from(variant: BOD_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl BOD_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOD_PD_A {
        match self.bits {
            false => BOD_PD_A::POWERED,
            true => BOD_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == BOD_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == BOD_PD_A::POWERED_DOWN
    }
}
#[doc = "Field `BOD_PD` writer - BOD power-down"]
pub type BOD_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG_SPEC, BOD_PD_A, O>;
impl<'a, const O: u8> BOD_PD_W<'a, O> {
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(BOD_PD_A::POWERED)
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(BOD_PD_A::POWERED_DOWN)
    }
}
#[doc = "Field `ADC_PD` reader - ADC power-down"]
pub type ADC_PD_R = crate::BitReader<ADC_PD_A>;
#[doc = "ADC power-down\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_PD_A {
    #[doc = "0: Powered"]
    POWERED = 0,
    #[doc = "1: Powered down"]
    POWERED_DOWN = 1,
}
impl From<ADC_PD_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_PD_A {
        match self.bits {
            false => ADC_PD_A::POWERED,
            true => ADC_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == ADC_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == ADC_PD_A::POWERED_DOWN
    }
}
#[doc = "Field `ADC_PD` writer - ADC power-down"]
pub type ADC_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG_SPEC, ADC_PD_A, O>;
impl<'a, const O: u8> ADC_PD_W<'a, O> {
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(ADC_PD_A::POWERED)
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(ADC_PD_A::POWERED_DOWN)
    }
}
#[doc = "Field `SYSOSC_PD` reader - Crystal oscillator power-down"]
pub type SYSOSC_PD_R = crate::BitReader<SYSOSC_PD_A>;
#[doc = "Crystal oscillator power-down\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSOSC_PD_A {
    #[doc = "0: Powered"]
    POWERED = 0,
    #[doc = "1: Powered down"]
    POWERED_DOWN = 1,
}
impl From<SYSOSC_PD_A> for bool {
    #[inline(always)]
    fn from(variant: SYSOSC_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSOSC_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSOSC_PD_A {
        match self.bits {
            false => SYSOSC_PD_A::POWERED,
            true => SYSOSC_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == SYSOSC_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == SYSOSC_PD_A::POWERED_DOWN
    }
}
#[doc = "Field `SYSOSC_PD` writer - Crystal oscillator power-down"]
pub type SYSOSC_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG_SPEC, SYSOSC_PD_A, O>;
impl<'a, const O: u8> SYSOSC_PD_W<'a, O> {
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(SYSOSC_PD_A::POWERED)
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(SYSOSC_PD_A::POWERED_DOWN)
    }
}
#[doc = "Field `WDTOSC_PD` reader - Watchdog oscillator power-down"]
pub type WDTOSC_PD_R = crate::BitReader<WDTOSC_PD_A>;
#[doc = "Watchdog oscillator power-down\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDTOSC_PD_A {
    #[doc = "0: Powered"]
    POWERED = 0,
    #[doc = "1: Powered down"]
    POWERED_DOWN = 1,
}
impl From<WDTOSC_PD_A> for bool {
    #[inline(always)]
    fn from(variant: WDTOSC_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl WDTOSC_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTOSC_PD_A {
        match self.bits {
            false => WDTOSC_PD_A::POWERED,
            true => WDTOSC_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == WDTOSC_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == WDTOSC_PD_A::POWERED_DOWN
    }
}
#[doc = "Field `WDTOSC_PD` writer - Watchdog oscillator power-down"]
pub type WDTOSC_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG_SPEC, WDTOSC_PD_A, O>;
impl<'a, const O: u8> WDTOSC_PD_W<'a, O> {
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(WDTOSC_PD_A::POWERED)
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(WDTOSC_PD_A::POWERED_DOWN)
    }
}
#[doc = "Field `SYSPLL_PD` reader - System PLL power-down"]
pub type SYSPLL_PD_R = crate::BitReader<SYSPLL_PD_A>;
#[doc = "System PLL power-down\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSPLL_PD_A {
    #[doc = "0: Powered"]
    POWERED = 0,
    #[doc = "1: Powered down"]
    POWERED_DOWN = 1,
}
impl From<SYSPLL_PD_A> for bool {
    #[inline(always)]
    fn from(variant: SYSPLL_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSPLL_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSPLL_PD_A {
        match self.bits {
            false => SYSPLL_PD_A::POWERED,
            true => SYSPLL_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == SYSPLL_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == SYSPLL_PD_A::POWERED_DOWN
    }
}
#[doc = "Field `SYSPLL_PD` writer - System PLL power-down"]
pub type SYSPLL_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG_SPEC, SYSPLL_PD_A, O>;
impl<'a, const O: u8> SYSPLL_PD_W<'a, O> {
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(SYSPLL_PD_A::POWERED)
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(SYSPLL_PD_A::POWERED_DOWN)
    }
}
#[doc = "Field `USBPLL_PD` reader - USB PLL power-down"]
pub type USBPLL_PD_R = crate::BitReader<USBPLL_PD_A>;
#[doc = "USB PLL power-down\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBPLL_PD_A {
    #[doc = "0: Powered"]
    POWERED = 0,
    #[doc = "1: Powered down"]
    POWERED_DOWN = 1,
}
impl From<USBPLL_PD_A> for bool {
    #[inline(always)]
    fn from(variant: USBPLL_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl USBPLL_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBPLL_PD_A {
        match self.bits {
            false => USBPLL_PD_A::POWERED,
            true => USBPLL_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == USBPLL_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == USBPLL_PD_A::POWERED_DOWN
    }
}
#[doc = "Field `USBPLL_PD` writer - USB PLL power-down"]
pub type USBPLL_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG_SPEC, USBPLL_PD_A, O>;
impl<'a, const O: u8> USBPLL_PD_W<'a, O> {
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(USBPLL_PD_A::POWERED)
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(USBPLL_PD_A::POWERED_DOWN)
    }
}
#[doc = "Field `USBPAD_PD` reader - USB transceiver power-down configuration"]
pub type USBPAD_PD_R = crate::BitReader<USBPAD_PD_A>;
#[doc = "USB transceiver power-down configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBPAD_PD_A {
    #[doc = "0: USB transceiver powered"]
    POWERED = 0,
    #[doc = "1: USB transceiver powered down (suspend mode)"]
    POWERED_DOWN = 1,
}
impl From<USBPAD_PD_A> for bool {
    #[inline(always)]
    fn from(variant: USBPAD_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl USBPAD_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBPAD_PD_A {
        match self.bits {
            false => USBPAD_PD_A::POWERED,
            true => USBPAD_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == USBPAD_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == USBPAD_PD_A::POWERED_DOWN
    }
}
#[doc = "Field `USBPAD_PD` writer - USB transceiver power-down configuration"]
pub type USBPAD_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG_SPEC, USBPAD_PD_A, O>;
impl<'a, const O: u8> USBPAD_PD_W<'a, O> {
    #[doc = "USB transceiver powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(USBPAD_PD_A::POWERED)
    }
    #[doc = "USB transceiver powered down (suspend mode)"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(USBPAD_PD_A::POWERED_DOWN)
    }
}
impl R {
    #[doc = "Bit 0 - IRC oscillator output power-down"]
    #[inline(always)]
    pub fn ircout_pd(&self) -> IRCOUT_PD_R {
        IRCOUT_PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRC oscillator power-down"]
    #[inline(always)]
    pub fn irc_pd(&self) -> IRC_PD_R {
        IRC_PD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Flash power-down"]
    #[inline(always)]
    pub fn flash_pd(&self) -> FLASH_PD_R {
        FLASH_PD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BOD power-down"]
    #[inline(always)]
    pub fn bod_pd(&self) -> BOD_PD_R {
        BOD_PD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC power-down"]
    #[inline(always)]
    pub fn adc_pd(&self) -> ADC_PD_R {
        ADC_PD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Crystal oscillator power-down"]
    #[inline(always)]
    pub fn sysosc_pd(&self) -> SYSOSC_PD_R {
        SYSOSC_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Watchdog oscillator power-down"]
    #[inline(always)]
    pub fn wdtosc_pd(&self) -> WDTOSC_PD_R {
        WDTOSC_PD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - System PLL power-down"]
    #[inline(always)]
    pub fn syspll_pd(&self) -> SYSPLL_PD_R {
        SYSPLL_PD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - USB PLL power-down"]
    #[inline(always)]
    pub fn usbpll_pd(&self) -> USBPLL_PD_R {
        USBPLL_PD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - USB transceiver power-down configuration"]
    #[inline(always)]
    pub fn usbpad_pd(&self) -> USBPAD_PD_R {
        USBPAD_PD_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IRC oscillator output power-down"]
    #[inline(always)]
    #[must_use]
    pub fn ircout_pd(&mut self) -> IRCOUT_PD_W<0> {
        IRCOUT_PD_W::new(self)
    }
    #[doc = "Bit 1 - IRC oscillator power-down"]
    #[inline(always)]
    #[must_use]
    pub fn irc_pd(&mut self) -> IRC_PD_W<1> {
        IRC_PD_W::new(self)
    }
    #[doc = "Bit 2 - Flash power-down"]
    #[inline(always)]
    #[must_use]
    pub fn flash_pd(&mut self) -> FLASH_PD_W<2> {
        FLASH_PD_W::new(self)
    }
    #[doc = "Bit 3 - BOD power-down"]
    #[inline(always)]
    #[must_use]
    pub fn bod_pd(&mut self) -> BOD_PD_W<3> {
        BOD_PD_W::new(self)
    }
    #[doc = "Bit 4 - ADC power-down"]
    #[inline(always)]
    #[must_use]
    pub fn adc_pd(&mut self) -> ADC_PD_W<4> {
        ADC_PD_W::new(self)
    }
    #[doc = "Bit 5 - Crystal oscillator power-down"]
    #[inline(always)]
    #[must_use]
    pub fn sysosc_pd(&mut self) -> SYSOSC_PD_W<5> {
        SYSOSC_PD_W::new(self)
    }
    #[doc = "Bit 6 - Watchdog oscillator power-down"]
    #[inline(always)]
    #[must_use]
    pub fn wdtosc_pd(&mut self) -> WDTOSC_PD_W<6> {
        WDTOSC_PD_W::new(self)
    }
    #[doc = "Bit 7 - System PLL power-down"]
    #[inline(always)]
    #[must_use]
    pub fn syspll_pd(&mut self) -> SYSPLL_PD_W<7> {
        SYSPLL_PD_W::new(self)
    }
    #[doc = "Bit 8 - USB PLL power-down"]
    #[inline(always)]
    #[must_use]
    pub fn usbpll_pd(&mut self) -> USBPLL_PD_W<8> {
        USBPLL_PD_W::new(self)
    }
    #[doc = "Bit 10 - USB transceiver power-down configuration"]
    #[inline(always)]
    #[must_use]
    pub fn usbpad_pd(&mut self) -> USBPAD_PD_W<10> {
        USBPAD_PD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdruncfg](index.html) module"]
pub struct PDRUNCFG_SPEC;
impl crate::RegisterSpec for PDRUNCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdruncfg::R](R) reader structure"]
impl crate::Readable for PDRUNCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdruncfg::W](W) writer structure"]
impl crate::Writable for PDRUNCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDRUNCFG to value 0xedf0"]
impl crate::Resettable for PDRUNCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0xedf0;
}
