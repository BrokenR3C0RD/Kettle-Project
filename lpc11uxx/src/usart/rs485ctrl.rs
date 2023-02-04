#[doc = "Register `RS485CTRL` reader"]
pub struct R(crate::R<RS485CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RS485CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RS485CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RS485CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RS485CTRL` writer"]
pub struct W(crate::W<RS485CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RS485CTRL_SPEC>;
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
impl From<crate::W<RS485CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RS485CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NMMEN` reader - NMM enable."]
pub type NMMEN_R = crate::BitReader<NMMEN_A>;
#[doc = "NMM enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NMMEN_A {
    #[doc = "0: RS-485/EIA-485 Normal Multidrop Mode (NMM) is disabled."]
    DISABLED = 0,
    #[doc = "1: RS-485/EIA-485 Normal Multidrop Mode (NMM) is enabled. In this mode, an address is detected when a received byte causes the USART to set the parity error and generate an interrupt."]
    ENABLED = 1,
}
impl From<NMMEN_A> for bool {
    #[inline(always)]
    fn from(variant: NMMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl NMMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NMMEN_A {
        match self.bits {
            false => NMMEN_A::DISABLED,
            true => NMMEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NMMEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NMMEN_A::ENABLED
    }
}
#[doc = "Field `NMMEN` writer - NMM enable."]
pub type NMMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RS485CTRL_SPEC, NMMEN_A, O>;
impl<'a, const O: u8> NMMEN_W<'a, O> {
    #[doc = "RS-485/EIA-485 Normal Multidrop Mode (NMM) is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NMMEN_A::DISABLED)
    }
    #[doc = "RS-485/EIA-485 Normal Multidrop Mode (NMM) is enabled. In this mode, an address is detected when a received byte causes the USART to set the parity error and generate an interrupt."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NMMEN_A::ENABLED)
    }
}
#[doc = "Field `RXDIS` reader - Receiver enable."]
pub type RXDIS_R = crate::BitReader<RXDIS_A>;
#[doc = "Receiver enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDIS_A {
    #[doc = "0: The receiver is enabled."]
    ENABLED = 0,
    #[doc = "1: The receiver is disabled."]
    DISABLED = 1,
}
impl From<RXDIS_A> for bool {
    #[inline(always)]
    fn from(variant: RXDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl RXDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDIS_A {
        match self.bits {
            false => RXDIS_A::ENABLED,
            true => RXDIS_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXDIS_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXDIS_A::DISABLED
    }
}
#[doc = "Field `RXDIS` writer - Receiver enable."]
pub type RXDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RS485CTRL_SPEC, RXDIS_A, O>;
impl<'a, const O: u8> RXDIS_W<'a, O> {
    #[doc = "The receiver is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXDIS_A::ENABLED)
    }
    #[doc = "The receiver is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXDIS_A::DISABLED)
    }
}
#[doc = "Field `AADEN` reader - AAD enable."]
pub type AADEN_R = crate::BitReader<AADEN_A>;
#[doc = "AAD enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AADEN_A {
    #[doc = "0: Auto Address Detect (AAD) is disabled."]
    DISABLED = 0,
    #[doc = "1: Auto Address Detect (AAD) is enabled."]
    ENABLED = 1,
}
impl From<AADEN_A> for bool {
    #[inline(always)]
    fn from(variant: AADEN_A) -> Self {
        variant as u8 != 0
    }
}
impl AADEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AADEN_A {
        match self.bits {
            false => AADEN_A::DISABLED,
            true => AADEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AADEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AADEN_A::ENABLED
    }
}
#[doc = "Field `AADEN` writer - AAD enable."]
pub type AADEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RS485CTRL_SPEC, AADEN_A, O>;
impl<'a, const O: u8> AADEN_W<'a, O> {
    #[doc = "Auto Address Detect (AAD) is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AADEN_A::DISABLED)
    }
    #[doc = "Auto Address Detect (AAD) is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AADEN_A::ENABLED)
    }
}
#[doc = "Field `SEL` reader - Select direction control pin"]
pub type SEL_R = crate::BitReader<SEL_A>;
#[doc = "Select direction control pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEL_A {
    #[doc = "0: If direction control is enabled (bit DCTRL = 1), pin RTS is used for direction control."]
    RTS = 0,
    #[doc = "1: If direction control is enabled (bit DCTRL = 1), pin DTR is used for direction control."]
    DTR = 1,
}
impl From<SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            false => SEL_A::RTS,
            true => SEL_A::DTR,
        }
    }
    #[doc = "Checks if the value of the field is `RTS`"]
    #[inline(always)]
    pub fn is_rts(&self) -> bool {
        *self == SEL_A::RTS
    }
    #[doc = "Checks if the value of the field is `DTR`"]
    #[inline(always)]
    pub fn is_dtr(&self) -> bool {
        *self == SEL_A::DTR
    }
}
#[doc = "Field `SEL` writer - Select direction control pin"]
pub type SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, RS485CTRL_SPEC, SEL_A, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "If direction control is enabled (bit DCTRL = 1), pin RTS is used for direction control."]
    #[inline(always)]
    pub fn rts(self) -> &'a mut W {
        self.variant(SEL_A::RTS)
    }
    #[doc = "If direction control is enabled (bit DCTRL = 1), pin DTR is used for direction control."]
    #[inline(always)]
    pub fn dtr(self) -> &'a mut W {
        self.variant(SEL_A::DTR)
    }
}
#[doc = "Field `DCTRL` reader - Auto direction control enable."]
pub type DCTRL_R = crate::BitReader<DCTRL_A>;
#[doc = "Auto direction control enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCTRL_A {
    #[doc = "0: Disable Auto Direction Control."]
    DISABLE = 0,
    #[doc = "1: Enable Auto Direction Control."]
    ENABLE = 1,
}
impl From<DCTRL_A> for bool {
    #[inline(always)]
    fn from(variant: DCTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl DCTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCTRL_A {
        match self.bits {
            false => DCTRL_A::DISABLE,
            true => DCTRL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DCTRL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DCTRL_A::ENABLE
    }
}
#[doc = "Field `DCTRL` writer - Auto direction control enable."]
pub type DCTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, RS485CTRL_SPEC, DCTRL_A, O>;
impl<'a, const O: u8> DCTRL_W<'a, O> {
    #[doc = "Disable Auto Direction Control."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DCTRL_A::DISABLE)
    }
    #[doc = "Enable Auto Direction Control."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DCTRL_A::ENABLE)
    }
}
#[doc = "Field `OINV` reader - Polarity control. This bit reverses the polarity of the direction control signal on the RTS (or DTR) pin."]
pub type OINV_R = crate::BitReader<OINV_A>;
#[doc = "Polarity control. This bit reverses the polarity of the direction control signal on the RTS (or DTR) pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OINV_A {
    #[doc = "0: The direction control pin will be driven to logic 0 when the transmitter has data to be sent. It will be driven to logic 1 after the last bit of data has been transmitted."]
    LOW = 0,
    #[doc = "1: The direction control pin will be driven to logic 1 when the transmitter has data to be sent. It will be driven to logic 0 after the last bit of data has been transmitted."]
    HIGH = 1,
}
impl From<OINV_A> for bool {
    #[inline(always)]
    fn from(variant: OINV_A) -> Self {
        variant as u8 != 0
    }
}
impl OINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OINV_A {
        match self.bits {
            false => OINV_A::LOW,
            true => OINV_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OINV_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OINV_A::HIGH
    }
}
#[doc = "Field `OINV` writer - Polarity control. This bit reverses the polarity of the direction control signal on the RTS (or DTR) pin."]
pub type OINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, RS485CTRL_SPEC, OINV_A, O>;
impl<'a, const O: u8> OINV_W<'a, O> {
    #[doc = "The direction control pin will be driven to logic 0 when the transmitter has data to be sent. It will be driven to logic 1 after the last bit of data has been transmitted."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OINV_A::LOW)
    }
    #[doc = "The direction control pin will be driven to logic 1 when the transmitter has data to be sent. It will be driven to logic 0 after the last bit of data has been transmitted."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OINV_A::HIGH)
    }
}
impl R {
    #[doc = "Bit 0 - NMM enable."]
    #[inline(always)]
    pub fn nmmen(&self) -> NMMEN_R {
        NMMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receiver enable."]
    #[inline(always)]
    pub fn rxdis(&self) -> RXDIS_R {
        RXDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AAD enable."]
    #[inline(always)]
    pub fn aaden(&self) -> AADEN_R {
        AADEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Select direction control pin"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Auto direction control enable."]
    #[inline(always)]
    pub fn dctrl(&self) -> DCTRL_R {
        DCTRL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Polarity control. This bit reverses the polarity of the direction control signal on the RTS (or DTR) pin."]
    #[inline(always)]
    pub fn oinv(&self) -> OINV_R {
        OINV_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NMM enable."]
    #[inline(always)]
    #[must_use]
    pub fn nmmen(&mut self) -> NMMEN_W<0> {
        NMMEN_W::new(self)
    }
    #[doc = "Bit 1 - Receiver enable."]
    #[inline(always)]
    #[must_use]
    pub fn rxdis(&mut self) -> RXDIS_W<1> {
        RXDIS_W::new(self)
    }
    #[doc = "Bit 2 - AAD enable."]
    #[inline(always)]
    #[must_use]
    pub fn aaden(&mut self) -> AADEN_W<2> {
        AADEN_W::new(self)
    }
    #[doc = "Bit 3 - Select direction control pin"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<3> {
        SEL_W::new(self)
    }
    #[doc = "Bit 4 - Auto direction control enable."]
    #[inline(always)]
    #[must_use]
    pub fn dctrl(&mut self) -> DCTRL_W<4> {
        DCTRL_W::new(self)
    }
    #[doc = "Bit 5 - Polarity control. This bit reverses the polarity of the direction control signal on the RTS (or DTR) pin."]
    #[inline(always)]
    #[must_use]
    pub fn oinv(&mut self) -> OINV_W<5> {
        OINV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RS-485/EIA-485 Control. Contains controls to configure various aspects of RS-485/EIA-485 modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rs485ctrl](index.html) module"]
pub struct RS485CTRL_SPEC;
impl crate::RegisterSpec for RS485CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rs485ctrl::R](R) reader structure"]
impl crate::Readable for RS485CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rs485ctrl::W](W) writer structure"]
impl crate::Writable for RS485CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RS485CTRL to value 0"]
impl crate::Resettable for RS485CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
