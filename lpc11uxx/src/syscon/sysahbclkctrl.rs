#[doc = "Register `SYSAHBCLKCTRL` reader"]
pub struct R(crate::R<SYSAHBCLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSAHBCLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSAHBCLKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSAHBCLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSAHBCLKCTRL` writer"]
pub struct W(crate::W<SYSAHBCLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSAHBCLKCTRL_SPEC>;
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
impl From<crate::W<SYSAHBCLKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSAHBCLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYS` reader - Enables the clock for the AHB, the APB bridge, the Cortex-M0 FCLK and HCLK, SysCon, and the PMU. This bit is read only and always reads as 1."]
pub type SYS_R = crate::BitReader<SYS_A>;
#[doc = "Enables the clock for the AHB, the APB bridge, the Cortex-M0 FCLK and HCLK, SysCon, and the PMU. This bit is read only and always reads as 1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYS_A {
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<SYS_A> for bool {
    #[inline(always)]
    fn from(variant: SYS_A) -> Self {
        variant as u8 != 0
    }
}
impl SYS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYS_A {
        match self.bits {
            true => SYS_A::ENABLED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYS_A::ENABLED
    }
}
#[doc = "Field `SYS` writer - Enables the clock for the AHB, the APB bridge, the Cortex-M0 FCLK and HCLK, SysCon, and the PMU. This bit is read only and always reads as 1."]
pub type SYS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, SYS_A, O>;
impl<'a, const O: u8> SYS_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SYS_A::ENABLED)
    }
}
#[doc = "Field `ROM` reader - Enables clock for ROM."]
pub type ROM_R = crate::BitReader<ROM_A>;
#[doc = "Enables clock for ROM.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROM_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ROM_A> for bool {
    #[inline(always)]
    fn from(variant: ROM_A) -> Self {
        variant as u8 != 0
    }
}
impl ROM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROM_A {
        match self.bits {
            false => ROM_A::DISABLED,
            true => ROM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ROM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ROM_A::ENABLED
    }
}
#[doc = "Field `ROM` writer - Enables clock for ROM."]
pub type ROM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, ROM_A, O>;
impl<'a, const O: u8> ROM_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ROM_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ROM_A::ENABLED)
    }
}
#[doc = "Field `RAM0` reader - Enables clock for RAM."]
pub type RAM0_R = crate::BitReader<RAM0_A>;
#[doc = "Enables clock for RAM.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAM0_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<RAM0_A> for bool {
    #[inline(always)]
    fn from(variant: RAM0_A) -> Self {
        variant as u8 != 0
    }
}
impl RAM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM0_A {
        match self.bits {
            false => RAM0_A::DISABLED,
            true => RAM0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RAM0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RAM0_A::ENABLED
    }
}
#[doc = "Field `RAM0` writer - Enables clock for RAM."]
pub type RAM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, RAM0_A, O>;
impl<'a, const O: u8> RAM0_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RAM0_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RAM0_A::ENABLED)
    }
}
#[doc = "Field `FLASHREG` reader - Enables clock for flash register interface."]
pub type FLASHREG_R = crate::BitReader<FLASHREG_A>;
#[doc = "Enables clock for flash register interface.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASHREG_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<FLASHREG_A> for bool {
    #[inline(always)]
    fn from(variant: FLASHREG_A) -> Self {
        variant as u8 != 0
    }
}
impl FLASHREG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASHREG_A {
        match self.bits {
            false => FLASHREG_A::DISABLED,
            true => FLASHREG_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FLASHREG_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FLASHREG_A::ENABLED
    }
}
#[doc = "Field `FLASHREG` writer - Enables clock for flash register interface."]
pub type FLASHREG_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, FLASHREG_A, O>;
impl<'a, const O: u8> FLASHREG_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLASHREG_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLASHREG_A::ENABLED)
    }
}
#[doc = "Field `FLASHARRAY` reader - Enables clock for flash array access."]
pub type FLASHARRAY_R = crate::BitReader<FLASHARRAY_A>;
#[doc = "Enables clock for flash array access.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASHARRAY_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<FLASHARRAY_A> for bool {
    #[inline(always)]
    fn from(variant: FLASHARRAY_A) -> Self {
        variant as u8 != 0
    }
}
impl FLASHARRAY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASHARRAY_A {
        match self.bits {
            false => FLASHARRAY_A::DISABLED,
            true => FLASHARRAY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FLASHARRAY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FLASHARRAY_A::ENABLED
    }
}
#[doc = "Field `FLASHARRAY` writer - Enables clock for flash array access."]
pub type FLASHARRAY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, FLASHARRAY_A, O>;
impl<'a, const O: u8> FLASHARRAY_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLASHARRAY_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLASHARRAY_A::ENABLED)
    }
}
#[doc = "Field `I2C` reader - Enables clock for I2C."]
pub type I2C_R = crate::BitReader<I2C_A>;
#[doc = "Enables clock for I2C.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
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
            false => I2C_A::DISABLED,
            true => I2C_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2C_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2C_A::ENABLED
    }
}
#[doc = "Field `I2C` writer - Enables clock for I2C."]
pub type I2C_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, I2C_A, O>;
impl<'a, const O: u8> I2C_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2C_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2C_A::ENABLED)
    }
}
#[doc = "Field `GPIO` reader - Enables clock for GPIO port registers."]
pub type GPIO_R = crate::BitReader<GPIO_A>;
#[doc = "Enables clock for GPIO port registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<GPIO_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_A {
        match self.bits {
            false => GPIO_A::DISABLED,
            true => GPIO_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO_A::ENABLED
    }
}
#[doc = "Field `GPIO` writer - Enables clock for GPIO port registers."]
pub type GPIO_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, GPIO_A, O>;
impl<'a, const O: u8> GPIO_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO_A::ENABLED)
    }
}
#[doc = "Field `CT16B0` reader - Enables clock for 16-bit counter/timer 0."]
pub type CT16B0_R = crate::BitReader<CT16B0_A>;
#[doc = "Enables clock for 16-bit counter/timer 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CT16B0_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<CT16B0_A> for bool {
    #[inline(always)]
    fn from(variant: CT16B0_A) -> Self {
        variant as u8 != 0
    }
}
impl CT16B0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CT16B0_A {
        match self.bits {
            false => CT16B0_A::DISABLED,
            true => CT16B0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CT16B0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CT16B0_A::ENABLED
    }
}
#[doc = "Field `CT16B0` writer - Enables clock for 16-bit counter/timer 0."]
pub type CT16B0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, CT16B0_A, O>;
impl<'a, const O: u8> CT16B0_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CT16B0_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CT16B0_A::ENABLED)
    }
}
#[doc = "Field `CT16B1` reader - Enables clock for 16-bit counter/timer 1."]
pub type CT16B1_R = crate::BitReader<CT16B1_A>;
#[doc = "Enables clock for 16-bit counter/timer 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CT16B1_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<CT16B1_A> for bool {
    #[inline(always)]
    fn from(variant: CT16B1_A) -> Self {
        variant as u8 != 0
    }
}
impl CT16B1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CT16B1_A {
        match self.bits {
            false => CT16B1_A::DISABLED,
            true => CT16B1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CT16B1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CT16B1_A::ENABLED
    }
}
#[doc = "Field `CT16B1` writer - Enables clock for 16-bit counter/timer 1."]
pub type CT16B1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, CT16B1_A, O>;
impl<'a, const O: u8> CT16B1_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CT16B1_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CT16B1_A::ENABLED)
    }
}
#[doc = "Field `CT32B0` reader - Enables clock for 32-bit counter/timer 0."]
pub type CT32B0_R = crate::BitReader<CT32B0_A>;
#[doc = "Enables clock for 32-bit counter/timer 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CT32B0_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<CT32B0_A> for bool {
    #[inline(always)]
    fn from(variant: CT32B0_A) -> Self {
        variant as u8 != 0
    }
}
impl CT32B0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CT32B0_A {
        match self.bits {
            false => CT32B0_A::DISABLED,
            true => CT32B0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CT32B0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CT32B0_A::ENABLED
    }
}
#[doc = "Field `CT32B0` writer - Enables clock for 32-bit counter/timer 0."]
pub type CT32B0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, CT32B0_A, O>;
impl<'a, const O: u8> CT32B0_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CT32B0_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CT32B0_A::ENABLED)
    }
}
#[doc = "Field `CT32B1` reader - Enables clock for 32-bit counter/timer 1."]
pub type CT32B1_R = crate::BitReader<CT32B1_A>;
#[doc = "Enables clock for 32-bit counter/timer 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CT32B1_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<CT32B1_A> for bool {
    #[inline(always)]
    fn from(variant: CT32B1_A) -> Self {
        variant as u8 != 0
    }
}
impl CT32B1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CT32B1_A {
        match self.bits {
            false => CT32B1_A::DISABLED,
            true => CT32B1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CT32B1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CT32B1_A::ENABLED
    }
}
#[doc = "Field `CT32B1` writer - Enables clock for 32-bit counter/timer 1."]
pub type CT32B1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, CT32B1_A, O>;
impl<'a, const O: u8> CT32B1_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CT32B1_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CT32B1_A::ENABLED)
    }
}
#[doc = "Field `SSP0` reader - Enables clock for SSP0."]
pub type SSP0_R = crate::BitReader<SSP0_A>;
#[doc = "Enables clock for SSP0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSP0_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
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
            false => SSP0_A::DISABLED,
            true => SSP0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSP0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSP0_A::ENABLED
    }
}
#[doc = "Field `SSP0` writer - Enables clock for SSP0."]
pub type SSP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, SSP0_A, O>;
impl<'a, const O: u8> SSP0_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSP0_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSP0_A::ENABLED)
    }
}
#[doc = "Field `USART` reader - Enables clock for UART."]
pub type USART_R = crate::BitReader<USART_A>;
#[doc = "Enables clock for UART.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<USART_A> for bool {
    #[inline(always)]
    fn from(variant: USART_A) -> Self {
        variant as u8 != 0
    }
}
impl USART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART_A {
        match self.bits {
            false => USART_A::DISABLED,
            true => USART_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USART_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USART_A::ENABLED
    }
}
#[doc = "Field `USART` writer - Enables clock for UART."]
pub type USART_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, USART_A, O>;
impl<'a, const O: u8> USART_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USART_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USART_A::ENABLED)
    }
}
#[doc = "Field `ADC` reader - Enables clock for ADC."]
pub type ADC_R = crate::BitReader<ADC_A>;
#[doc = "Enables clock for ADC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ADC_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_A {
        match self.bits {
            false => ADC_A::DISABLED,
            true => ADC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_A::ENABLED
    }
}
#[doc = "Field `ADC` writer - Enables clock for ADC."]
pub type ADC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, ADC_A, O>;
impl<'a, const O: u8> ADC_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_A::ENABLED)
    }
}
#[doc = "Field `USB` reader - Enables clock to the USB register interface."]
pub type USB_R = crate::BitReader<USB_A>;
#[doc = "Enables clock to the USB register interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USB_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<USB_A> for bool {
    #[inline(always)]
    fn from(variant: USB_A) -> Self {
        variant as u8 != 0
    }
}
impl USB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB_A {
        match self.bits {
            false => USB_A::DISABLED,
            true => USB_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USB_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USB_A::ENABLED
    }
}
#[doc = "Field `USB` writer - Enables clock to the USB register interface."]
pub type USB_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, USB_A, O>;
impl<'a, const O: u8> USB_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USB_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USB_A::ENABLED)
    }
}
#[doc = "Field `WWDT` reader - Enables clock for WWDT."]
pub type WWDT_R = crate::BitReader<WWDT_A>;
#[doc = "Enables clock for WWDT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDT_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<WWDT_A> for bool {
    #[inline(always)]
    fn from(variant: WWDT_A) -> Self {
        variant as u8 != 0
    }
}
impl WWDT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WWDT_A {
        match self.bits {
            false => WWDT_A::DISABLED,
            true => WWDT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WWDT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WWDT_A::ENABLED
    }
}
#[doc = "Field `WWDT` writer - Enables clock for WWDT."]
pub type WWDT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, WWDT_A, O>;
impl<'a, const O: u8> WWDT_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WWDT_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WWDT_A::ENABLED)
    }
}
#[doc = "Field `IOCON` reader - Enables clock for I/O configuration block."]
pub type IOCON_R = crate::BitReader<IOCON_A>;
#[doc = "Enables clock for I/O configuration block.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOCON_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<IOCON_A> for bool {
    #[inline(always)]
    fn from(variant: IOCON_A) -> Self {
        variant as u8 != 0
    }
}
impl IOCON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOCON_A {
        match self.bits {
            false => IOCON_A::DISABLED,
            true => IOCON_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IOCON_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IOCON_A::ENABLED
    }
}
#[doc = "Field `IOCON` writer - Enables clock for I/O configuration block."]
pub type IOCON_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, IOCON_A, O>;
impl<'a, const O: u8> IOCON_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IOCON_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IOCON_A::ENABLED)
    }
}
#[doc = "Field `SSP1` reader - Enables clock for SSP1."]
pub type SSP1_R = crate::BitReader<SSP1_A>;
#[doc = "Enables clock for SSP1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSP1_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
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
            false => SSP1_A::DISABLED,
            true => SSP1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSP1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSP1_A::ENABLED
    }
}
#[doc = "Field `SSP1` writer - Enables clock for SSP1."]
pub type SSP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, SSP1_A, O>;
impl<'a, const O: u8> SSP1_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSP1_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSP1_A::ENABLED)
    }
}
#[doc = "Field `PINT` reader - Enables clock to GPIO Pin interrupts register interface."]
pub type PINT_R = crate::BitReader<PINT_A>;
#[doc = "Enables clock to GPIO Pin interrupts register interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINT_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<PINT_A> for bool {
    #[inline(always)]
    fn from(variant: PINT_A) -> Self {
        variant as u8 != 0
    }
}
impl PINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINT_A {
        match self.bits {
            false => PINT_A::DISABLED,
            true => PINT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PINT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PINT_A::ENABLED
    }
}
#[doc = "Field `PINT` writer - Enables clock to GPIO Pin interrupts register interface."]
pub type PINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, PINT_A, O>;
impl<'a, const O: u8> PINT_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PINT_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PINT_A::ENABLED)
    }
}
#[doc = "Field `GROUP0INT` reader - Enables clock to GPIO GROUP0 interrupt register interface."]
pub type GROUP0INT_R = crate::BitReader<GROUP0INT_A>;
#[doc = "Enables clock to GPIO GROUP0 interrupt register interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GROUP0INT_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<GROUP0INT_A> for bool {
    #[inline(always)]
    fn from(variant: GROUP0INT_A) -> Self {
        variant as u8 != 0
    }
}
impl GROUP0INT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GROUP0INT_A {
        match self.bits {
            false => GROUP0INT_A::DISABLED,
            true => GROUP0INT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GROUP0INT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GROUP0INT_A::ENABLED
    }
}
#[doc = "Field `GROUP0INT` writer - Enables clock to GPIO GROUP0 interrupt register interface."]
pub type GROUP0INT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, GROUP0INT_A, O>;
impl<'a, const O: u8> GROUP0INT_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GROUP0INT_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GROUP0INT_A::ENABLED)
    }
}
#[doc = "Field `GROUP1INT` reader - Enables clock to GPIO GROUP1 interrupt register interface."]
pub type GROUP1INT_R = crate::BitReader<GROUP1INT_A>;
#[doc = "Enables clock to GPIO GROUP1 interrupt register interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GROUP1INT_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<GROUP1INT_A> for bool {
    #[inline(always)]
    fn from(variant: GROUP1INT_A) -> Self {
        variant as u8 != 0
    }
}
impl GROUP1INT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GROUP1INT_A {
        match self.bits {
            false => GROUP1INT_A::DISABLED,
            true => GROUP1INT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GROUP1INT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GROUP1INT_A::ENABLED
    }
}
#[doc = "Field `GROUP1INT` writer - Enables clock to GPIO GROUP1 interrupt register interface."]
pub type GROUP1INT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, GROUP1INT_A, O>;
impl<'a, const O: u8> GROUP1INT_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GROUP1INT_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GROUP1INT_A::ENABLED)
    }
}
#[doc = "Field `RAM1` reader - Enables SRAM1 block at address 0x2000 0000. See Section 3.1 for availability of this bit."]
pub type RAM1_R = crate::BitReader<RAM1_A>;
#[doc = "Enables SRAM1 block at address 0x2000 0000. See Section 3.1 for availability of this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAM1_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<RAM1_A> for bool {
    #[inline(always)]
    fn from(variant: RAM1_A) -> Self {
        variant as u8 != 0
    }
}
impl RAM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM1_A {
        match self.bits {
            false => RAM1_A::DISABLED,
            true => RAM1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RAM1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RAM1_A::ENABLED
    }
}
#[doc = "Field `RAM1` writer - Enables SRAM1 block at address 0x2000 0000. See Section 3.1 for availability of this bit."]
pub type RAM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, RAM1_A, O>;
impl<'a, const O: u8> RAM1_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RAM1_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RAM1_A::ENABLED)
    }
}
#[doc = "Field `USBRAM` reader - Enables USB SRAM block at address 0x2000 4000."]
pub type USBRAM_R = crate::BitReader<USBRAM_A>;
#[doc = "Enables USB SRAM block at address 0x2000 4000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBRAM_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<USBRAM_A> for bool {
    #[inline(always)]
    fn from(variant: USBRAM_A) -> Self {
        variant as u8 != 0
    }
}
impl USBRAM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBRAM_A {
        match self.bits {
            false => USBRAM_A::DISABLED,
            true => USBRAM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USBRAM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USBRAM_A::ENABLED
    }
}
#[doc = "Field `USBRAM` writer - Enables USB SRAM block at address 0x2000 4000."]
pub type USBRAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, USBRAM_A, O>;
impl<'a, const O: u8> USBRAM_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USBRAM_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USBRAM_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Enables the clock for the AHB, the APB bridge, the Cortex-M0 FCLK and HCLK, SysCon, and the PMU. This bit is read only and always reads as 1."]
    #[inline(always)]
    pub fn sys(&self) -> SYS_R {
        SYS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables clock for ROM."]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables clock for RAM."]
    #[inline(always)]
    pub fn ram0(&self) -> RAM0_R {
        RAM0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables clock for flash register interface."]
    #[inline(always)]
    pub fn flashreg(&self) -> FLASHREG_R {
        FLASHREG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables clock for flash array access."]
    #[inline(always)]
    pub fn flasharray(&self) -> FLASHARRAY_R {
        FLASHARRAY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enables clock for I2C."]
    #[inline(always)]
    pub fn i2c(&self) -> I2C_R {
        I2C_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enables clock for GPIO port registers."]
    #[inline(always)]
    pub fn gpio(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enables clock for 16-bit counter/timer 0."]
    #[inline(always)]
    pub fn ct16b0(&self) -> CT16B0_R {
        CT16B0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enables clock for 16-bit counter/timer 1."]
    #[inline(always)]
    pub fn ct16b1(&self) -> CT16B1_R {
        CT16B1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enables clock for 32-bit counter/timer 0."]
    #[inline(always)]
    pub fn ct32b0(&self) -> CT32B0_R {
        CT32B0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enables clock for 32-bit counter/timer 1."]
    #[inline(always)]
    pub fn ct32b1(&self) -> CT32B1_R {
        CT32B1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enables clock for SSP0."]
    #[inline(always)]
    pub fn ssp0(&self) -> SSP0_R {
        SSP0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enables clock for UART."]
    #[inline(always)]
    pub fn usart(&self) -> USART_R {
        USART_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enables clock for ADC."]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enables clock to the USB register interface."]
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enables clock for WWDT."]
    #[inline(always)]
    pub fn wwdt(&self) -> WWDT_R {
        WWDT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enables clock for I/O configuration block."]
    #[inline(always)]
    pub fn iocon(&self) -> IOCON_R {
        IOCON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Enables clock for SSP1."]
    #[inline(always)]
    pub fn ssp1(&self) -> SSP1_R {
        SSP1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enables clock to GPIO Pin interrupts register interface."]
    #[inline(always)]
    pub fn pint(&self) -> PINT_R {
        PINT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 23 - Enables clock to GPIO GROUP0 interrupt register interface."]
    #[inline(always)]
    pub fn group0int(&self) -> GROUP0INT_R {
        GROUP0INT_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enables clock to GPIO GROUP1 interrupt register interface."]
    #[inline(always)]
    pub fn group1int(&self) -> GROUP1INT_R {
        GROUP1INT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Enables SRAM1 block at address 0x2000 0000. See Section 3.1 for availability of this bit."]
    #[inline(always)]
    pub fn ram1(&self) -> RAM1_R {
        RAM1_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enables USB SRAM block at address 0x2000 4000."]
    #[inline(always)]
    pub fn usbram(&self) -> USBRAM_R {
        USBRAM_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the clock for the AHB, the APB bridge, the Cortex-M0 FCLK and HCLK, SysCon, and the PMU. This bit is read only and always reads as 1."]
    #[inline(always)]
    #[must_use]
    pub fn sys(&mut self) -> SYS_W<0> {
        SYS_W::new(self)
    }
    #[doc = "Bit 1 - Enables clock for ROM."]
    #[inline(always)]
    #[must_use]
    pub fn rom(&mut self) -> ROM_W<1> {
        ROM_W::new(self)
    }
    #[doc = "Bit 2 - Enables clock for RAM."]
    #[inline(always)]
    #[must_use]
    pub fn ram0(&mut self) -> RAM0_W<2> {
        RAM0_W::new(self)
    }
    #[doc = "Bit 3 - Enables clock for flash register interface."]
    #[inline(always)]
    #[must_use]
    pub fn flashreg(&mut self) -> FLASHREG_W<3> {
        FLASHREG_W::new(self)
    }
    #[doc = "Bit 4 - Enables clock for flash array access."]
    #[inline(always)]
    #[must_use]
    pub fn flasharray(&mut self) -> FLASHARRAY_W<4> {
        FLASHARRAY_W::new(self)
    }
    #[doc = "Bit 5 - Enables clock for I2C."]
    #[inline(always)]
    #[must_use]
    pub fn i2c(&mut self) -> I2C_W<5> {
        I2C_W::new(self)
    }
    #[doc = "Bit 6 - Enables clock for GPIO port registers."]
    #[inline(always)]
    #[must_use]
    pub fn gpio(&mut self) -> GPIO_W<6> {
        GPIO_W::new(self)
    }
    #[doc = "Bit 7 - Enables clock for 16-bit counter/timer 0."]
    #[inline(always)]
    #[must_use]
    pub fn ct16b0(&mut self) -> CT16B0_W<7> {
        CT16B0_W::new(self)
    }
    #[doc = "Bit 8 - Enables clock for 16-bit counter/timer 1."]
    #[inline(always)]
    #[must_use]
    pub fn ct16b1(&mut self) -> CT16B1_W<8> {
        CT16B1_W::new(self)
    }
    #[doc = "Bit 9 - Enables clock for 32-bit counter/timer 0."]
    #[inline(always)]
    #[must_use]
    pub fn ct32b0(&mut self) -> CT32B0_W<9> {
        CT32B0_W::new(self)
    }
    #[doc = "Bit 10 - Enables clock for 32-bit counter/timer 1."]
    #[inline(always)]
    #[must_use]
    pub fn ct32b1(&mut self) -> CT32B1_W<10> {
        CT32B1_W::new(self)
    }
    #[doc = "Bit 11 - Enables clock for SSP0."]
    #[inline(always)]
    #[must_use]
    pub fn ssp0(&mut self) -> SSP0_W<11> {
        SSP0_W::new(self)
    }
    #[doc = "Bit 12 - Enables clock for UART."]
    #[inline(always)]
    #[must_use]
    pub fn usart(&mut self) -> USART_W<12> {
        USART_W::new(self)
    }
    #[doc = "Bit 13 - Enables clock for ADC."]
    #[inline(always)]
    #[must_use]
    pub fn adc(&mut self) -> ADC_W<13> {
        ADC_W::new(self)
    }
    #[doc = "Bit 14 - Enables clock to the USB register interface."]
    #[inline(always)]
    #[must_use]
    pub fn usb(&mut self) -> USB_W<14> {
        USB_W::new(self)
    }
    #[doc = "Bit 15 - Enables clock for WWDT."]
    #[inline(always)]
    #[must_use]
    pub fn wwdt(&mut self) -> WWDT_W<15> {
        WWDT_W::new(self)
    }
    #[doc = "Bit 16 - Enables clock for I/O configuration block."]
    #[inline(always)]
    #[must_use]
    pub fn iocon(&mut self) -> IOCON_W<16> {
        IOCON_W::new(self)
    }
    #[doc = "Bit 18 - Enables clock for SSP1."]
    #[inline(always)]
    #[must_use]
    pub fn ssp1(&mut self) -> SSP1_W<18> {
        SSP1_W::new(self)
    }
    #[doc = "Bit 19 - Enables clock to GPIO Pin interrupts register interface."]
    #[inline(always)]
    #[must_use]
    pub fn pint(&mut self) -> PINT_W<19> {
        PINT_W::new(self)
    }
    #[doc = "Bit 23 - Enables clock to GPIO GROUP0 interrupt register interface."]
    #[inline(always)]
    #[must_use]
    pub fn group0int(&mut self) -> GROUP0INT_W<23> {
        GROUP0INT_W::new(self)
    }
    #[doc = "Bit 24 - Enables clock to GPIO GROUP1 interrupt register interface."]
    #[inline(always)]
    #[must_use]
    pub fn group1int(&mut self) -> GROUP1INT_W<24> {
        GROUP1INT_W::new(self)
    }
    #[doc = "Bit 26 - Enables SRAM1 block at address 0x2000 0000. See Section 3.1 for availability of this bit."]
    #[inline(always)]
    #[must_use]
    pub fn ram1(&mut self) -> RAM1_W<26> {
        RAM1_W::new(self)
    }
    #[doc = "Bit 27 - Enables USB SRAM block at address 0x2000 4000."]
    #[inline(always)]
    #[must_use]
    pub fn usbram(&mut self) -> USBRAM_W<27> {
        USBRAM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysahbclkctrl](index.html) module"]
pub struct SYSAHBCLKCTRL_SPEC;
impl crate::RegisterSpec for SYSAHBCLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysahbclkctrl::R](R) reader structure"]
impl crate::Readable for SYSAHBCLKCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysahbclkctrl::W](W) writer structure"]
impl crate::Writable for SYSAHBCLKCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSAHBCLKCTRL to value 0x3f"]
impl crate::Resettable for SYSAHBCLKCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f;
}
