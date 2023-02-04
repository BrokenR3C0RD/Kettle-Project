#[doc = "Register `SYNCCTRL` reader"]
pub struct R(crate::R<SYNCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYNCCTRL` writer"]
pub struct W(crate::W<SYNCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYNCCTRL_SPEC>;
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
impl From<crate::W<SYNCCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYNCCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYNC` reader - Enables synchronous mode."]
pub type SYNC_R = crate::BitReader<SYNC_A>;
#[doc = "Enables synchronous mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNC_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<SYNC_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC_A {
        match self.bits {
            false => SYNC_A::DISABLED,
            true => SYNC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYNC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYNC_A::ENABLED
    }
}
#[doc = "Field `SYNC` writer - Enables synchronous mode."]
pub type SYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYNCCTRL_SPEC, SYNC_A, O>;
impl<'a, const O: u8> SYNC_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYNC_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SYNC_A::ENABLED)
    }
}
#[doc = "Field `CSRC` reader - Clock source select."]
pub type CSRC_R = crate::BitReader<CSRC_A>;
#[doc = "Clock source select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSRC_A {
    #[doc = "0: Synchronous slave mode (SCLK in)"]
    SLAVE = 0,
    #[doc = "1: Synchronous master mode (SCLK out)"]
    MASTER = 1,
}
impl From<CSRC_A> for bool {
    #[inline(always)]
    fn from(variant: CSRC_A) -> Self {
        variant as u8 != 0
    }
}
impl CSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSRC_A {
        match self.bits {
            false => CSRC_A::SLAVE,
            true => CSRC_A::MASTER,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == CSRC_A::SLAVE
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == CSRC_A::MASTER
    }
}
#[doc = "Field `CSRC` writer - Clock source select."]
pub type CSRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYNCCTRL_SPEC, CSRC_A, O>;
impl<'a, const O: u8> CSRC_W<'a, O> {
    #[doc = "Synchronous slave mode (SCLK in)"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut W {
        self.variant(CSRC_A::SLAVE)
    }
    #[doc = "Synchronous master mode (SCLK out)"]
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(CSRC_A::MASTER)
    }
}
#[doc = "Field `FES` reader - Falling edge sampling."]
pub type FES_R = crate::BitReader<FES_A>;
#[doc = "Falling edge sampling.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FES_A {
    #[doc = "0: RxD is sampled on the rising edge of SCLK"]
    RISING = 0,
    #[doc = "1: RxD is sampled on the falling edge of SCLK"]
    FALLING = 1,
}
impl From<FES_A> for bool {
    #[inline(always)]
    fn from(variant: FES_A) -> Self {
        variant as u8 != 0
    }
}
impl FES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FES_A {
        match self.bits {
            false => FES_A::RISING,
            true => FES_A::FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == FES_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == FES_A::FALLING
    }
}
#[doc = "Field `FES` writer - Falling edge sampling."]
pub type FES_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYNCCTRL_SPEC, FES_A, O>;
impl<'a, const O: u8> FES_W<'a, O> {
    #[doc = "RxD is sampled on the rising edge of SCLK"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(FES_A::RISING)
    }
    #[doc = "RxD is sampled on the falling edge of SCLK"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(FES_A::FALLING)
    }
}
#[doc = "Field `TSBYPASS` reader - Transmit synchronization bypass in synchronous slave mode."]
pub type TSBYPASS_R = crate::BitReader<TSBYPASS_A>;
#[doc = "Transmit synchronization bypass in synchronous slave mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSBYPASS_A {
    #[doc = "0: The input clock is synchronized prior to being used in clock edge detection logic"]
    SYNC = 0,
    #[doc = "1: The input clock is not synchronized prior to being used in clock edge detection logic. This allows for a high er input clock rate at the expense of potential metastability."]
    NOSYNC = 1,
}
impl From<TSBYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: TSBYPASS_A) -> Self {
        variant as u8 != 0
    }
}
impl TSBYPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSBYPASS_A {
        match self.bits {
            false => TSBYPASS_A::SYNC,
            true => TSBYPASS_A::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `SYNC`"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == TSBYPASS_A::SYNC
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline(always)]
    pub fn is_nosync(&self) -> bool {
        *self == TSBYPASS_A::NOSYNC
    }
}
#[doc = "Field `TSBYPASS` writer - Transmit synchronization bypass in synchronous slave mode."]
pub type TSBYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYNCCTRL_SPEC, TSBYPASS_A, O>;
impl<'a, const O: u8> TSBYPASS_W<'a, O> {
    #[doc = "The input clock is synchronized prior to being used in clock edge detection logic"]
    #[inline(always)]
    pub fn sync(self) -> &'a mut W {
        self.variant(TSBYPASS_A::SYNC)
    }
    #[doc = "The input clock is not synchronized prior to being used in clock edge detection logic. This allows for a high er input clock rate at the expense of potential metastability."]
    #[inline(always)]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TSBYPASS_A::NOSYNC)
    }
}
#[doc = "Field `CSCEN` reader - Continuous master clock enable (used only when CSRC is 1)"]
pub type CSCEN_R = crate::BitReader<CSCEN_A>;
#[doc = "Continuous master clock enable (used only when CSRC is 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSCEN_A {
    #[doc = "0: SCLK cycles only when characters are being sent on TxD"]
    DISABLED = 0,
    #[doc = "1: SCLK runs continuously (characters can be received on RxD independently from transmission on TxD)"]
    ENABLED = 1,
}
impl From<CSCEN_A> for bool {
    #[inline(always)]
    fn from(variant: CSCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CSCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSCEN_A {
        match self.bits {
            false => CSCEN_A::DISABLED,
            true => CSCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CSCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CSCEN_A::ENABLED
    }
}
#[doc = "Field `CSCEN` writer - Continuous master clock enable (used only when CSRC is 1)"]
pub type CSCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYNCCTRL_SPEC, CSCEN_A, O>;
impl<'a, const O: u8> CSCEN_W<'a, O> {
    #[doc = "SCLK cycles only when characters are being sent on TxD"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CSCEN_A::DISABLED)
    }
    #[doc = "SCLK runs continuously (characters can be received on RxD independently from transmission on TxD)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CSCEN_A::ENABLED)
    }
}
#[doc = "Field `SSDIS` reader - Start/stop bits"]
pub type SSDIS_R = crate::BitReader<SSDIS_A>;
#[doc = "Start/stop bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSDIS_A {
    #[doc = "0: Send start and stop bits as in other modes."]
    SEND = 0,
    #[doc = "1: Do not send start/stop bits."]
    DONT_SEND = 1,
}
impl From<SSDIS_A> for bool {
    #[inline(always)]
    fn from(variant: SSDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl SSDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSDIS_A {
        match self.bits {
            false => SSDIS_A::SEND,
            true => SSDIS_A::DONT_SEND,
        }
    }
    #[doc = "Checks if the value of the field is `SEND`"]
    #[inline(always)]
    pub fn is_send(&self) -> bool {
        *self == SSDIS_A::SEND
    }
    #[doc = "Checks if the value of the field is `DONT_SEND`"]
    #[inline(always)]
    pub fn is_dont_send(&self) -> bool {
        *self == SSDIS_A::DONT_SEND
    }
}
#[doc = "Field `SSDIS` writer - Start/stop bits"]
pub type SSDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYNCCTRL_SPEC, SSDIS_A, O>;
impl<'a, const O: u8> SSDIS_W<'a, O> {
    #[doc = "Send start and stop bits as in other modes."]
    #[inline(always)]
    pub fn send(self) -> &'a mut W {
        self.variant(SSDIS_A::SEND)
    }
    #[doc = "Do not send start/stop bits."]
    #[inline(always)]
    pub fn dont_send(self) -> &'a mut W {
        self.variant(SSDIS_A::DONT_SEND)
    }
}
#[doc = "Field `CCCLR` reader - Continuous clock clear"]
pub type CCCLR_R = crate::BitReader<CCCLR_A>;
#[doc = "Continuous clock clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCCLR_A {
    #[doc = "0: CSCEN is under software control."]
    SOFTWARE = 0,
    #[doc = "1: Hardware clears CSCEN after each character is received."]
    HARDWARE = 1,
}
impl From<CCCLR_A> for bool {
    #[inline(always)]
    fn from(variant: CCCLR_A) -> Self {
        variant as u8 != 0
    }
}
impl CCCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCCLR_A {
        match self.bits {
            false => CCCLR_A::SOFTWARE,
            true => CCCLR_A::HARDWARE,
        }
    }
    #[doc = "Checks if the value of the field is `SOFTWARE`"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == CCCLR_A::SOFTWARE
    }
    #[doc = "Checks if the value of the field is `HARDWARE`"]
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == CCCLR_A::HARDWARE
    }
}
#[doc = "Field `CCCLR` writer - Continuous clock clear"]
pub type CCCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYNCCTRL_SPEC, CCCLR_A, O>;
impl<'a, const O: u8> CCCLR_W<'a, O> {
    #[doc = "CSCEN is under software control."]
    #[inline(always)]
    pub fn software(self) -> &'a mut W {
        self.variant(CCCLR_A::SOFTWARE)
    }
    #[doc = "Hardware clears CSCEN after each character is received."]
    #[inline(always)]
    pub fn hardware(self) -> &'a mut W {
        self.variant(CCCLR_A::HARDWARE)
    }
}
impl R {
    #[doc = "Bit 0 - Enables synchronous mode."]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock source select."]
    #[inline(always)]
    pub fn csrc(&self) -> CSRC_R {
        CSRC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Falling edge sampling."]
    #[inline(always)]
    pub fn fes(&self) -> FES_R {
        FES_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit synchronization bypass in synchronous slave mode."]
    #[inline(always)]
    pub fn tsbypass(&self) -> TSBYPASS_R {
        TSBYPASS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Continuous master clock enable (used only when CSRC is 1)"]
    #[inline(always)]
    pub fn cscen(&self) -> CSCEN_R {
        CSCEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Start/stop bits"]
    #[inline(always)]
    pub fn ssdis(&self) -> SSDIS_R {
        SSDIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Continuous clock clear"]
    #[inline(always)]
    pub fn ccclr(&self) -> CCCLR_R {
        CCCLR_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables synchronous mode."]
    #[inline(always)]
    #[must_use]
    pub fn sync(&mut self) -> SYNC_W<0> {
        SYNC_W::new(self)
    }
    #[doc = "Bit 1 - Clock source select."]
    #[inline(always)]
    #[must_use]
    pub fn csrc(&mut self) -> CSRC_W<1> {
        CSRC_W::new(self)
    }
    #[doc = "Bit 2 - Falling edge sampling."]
    #[inline(always)]
    #[must_use]
    pub fn fes(&mut self) -> FES_W<2> {
        FES_W::new(self)
    }
    #[doc = "Bit 3 - Transmit synchronization bypass in synchronous slave mode."]
    #[inline(always)]
    #[must_use]
    pub fn tsbypass(&mut self) -> TSBYPASS_W<3> {
        TSBYPASS_W::new(self)
    }
    #[doc = "Bit 4 - Continuous master clock enable (used only when CSRC is 1)"]
    #[inline(always)]
    #[must_use]
    pub fn cscen(&mut self) -> CSCEN_W<4> {
        CSCEN_W::new(self)
    }
    #[doc = "Bit 5 - Start/stop bits"]
    #[inline(always)]
    #[must_use]
    pub fn ssdis(&mut self) -> SSDIS_W<5> {
        SSDIS_W::new(self)
    }
    #[doc = "Bit 6 - Continuous clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn ccclr(&mut self) -> CCCLR_W<6> {
        CCCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Synchronous mode control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncctrl](index.html) module"]
pub struct SYNCCTRL_SPEC;
impl crate::RegisterSpec for SYNCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syncctrl::R](R) reader structure"]
impl crate::Readable for SYNCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syncctrl::W](W) writer structure"]
impl crate::Writable for SYNCCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYNCCTRL to value 0"]
impl crate::Resettable for SYNCCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
