#[doc = "Register `DEVCMDSTAT` reader"]
pub struct R(crate::R<DEVCMDSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVCMDSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVCMDSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVCMDSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEVCMDSTAT` writer"]
pub struct W(crate::W<DEVCMDSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVCMDSTAT_SPEC>;
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
impl From<crate::W<DEVCMDSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVCMDSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEV_ADDR` reader - USB device address. After bus reset, the address is reset to 0x00. If the enable bit is set, the device will respond on packets for function address DEV_ADDR. When receiving a SetAddress Control Request from the USB host, software must program the new address before completing the status phase of the SetAddress Control Request."]
pub type DEV_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEV_ADDR` writer - USB device address. After bus reset, the address is reset to 0x00. If the enable bit is set, the device will respond on packets for function address DEV_ADDR. When receiving a SetAddress Control Request from the USB host, software must program the new address before completing the status phase of the SetAddress Control Request."]
pub type DEV_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEVCMDSTAT_SPEC, u8, u8, 7, O>;
#[doc = "Field `DEV_EN` reader - USB device enable. If this bit is set, the HW will start responding on packets for function address DEV_ADDR."]
pub type DEV_EN_R = crate::BitReader<bool>;
#[doc = "Field `DEV_EN` writer - USB device enable. If this bit is set, the HW will start responding on packets for function address DEV_ADDR."]
pub type DEV_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, bool, O>;
#[doc = "Field `SETUP` reader - SETUP token received. If a SETUP token is received and acknowledged by the device, this bit is set. As long as this bit is set all received IN and OUT tokens will be NAKed by HW. SW must clear this bit by writing a one. If this bit is zero, HW will handle the tokens to the CTRL EP0 as indicated by the CTRL EP0 IN and OUT data information programmed by SW."]
pub type SETUP_R = crate::BitReader<bool>;
#[doc = "Field `SETUP` writer - SETUP token received. If a SETUP token is received and acknowledged by the device, this bit is set. As long as this bit is set all received IN and OUT tokens will be NAKed by HW. SW must clear this bit by writing a one. If this bit is zero, HW will handle the tokens to the CTRL EP0 as indicated by the CTRL EP0 IN and OUT data information programmed by SW."]
pub type SETUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, bool, O>;
#[doc = "Field `PLL_ON` reader - Always PLL Clock on:"]
pub type PLL_ON_R = crate::BitReader<PLL_ON_A>;
#[doc = "Always PLL Clock on:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL_ON_A {
    #[doc = "0: USB_NeedClk functional"]
    HARDWARE = 0,
    #[doc = "1: USB_NeedClk always 1. Clock will not be stopped in case of suspend."]
    ALWAYS_ON = 1,
}
impl From<PLL_ON_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_ON_A) -> Self {
        variant as u8 != 0
    }
}
impl PLL_ON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL_ON_A {
        match self.bits {
            false => PLL_ON_A::HARDWARE,
            true => PLL_ON_A::ALWAYS_ON,
        }
    }
    #[doc = "Checks if the value of the field is `HARDWARE`"]
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == PLL_ON_A::HARDWARE
    }
    #[doc = "Checks if the value of the field is `ALWAYS_ON`"]
    #[inline(always)]
    pub fn is_always_on(&self) -> bool {
        *self == PLL_ON_A::ALWAYS_ON
    }
}
#[doc = "Field `PLL_ON` writer - Always PLL Clock on:"]
pub type PLL_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, PLL_ON_A, O>;
impl<'a, const O: u8> PLL_ON_W<'a, O> {
    #[doc = "USB_NeedClk functional"]
    #[inline(always)]
    pub fn hardware(self) -> &'a mut W {
        self.variant(PLL_ON_A::HARDWARE)
    }
    #[doc = "USB_NeedClk always 1. Clock will not be stopped in case of suspend."]
    #[inline(always)]
    pub fn always_on(self) -> &'a mut W {
        self.variant(PLL_ON_A::ALWAYS_ON)
    }
}
#[doc = "Field `LPM_SUP` reader - LPM Supported:"]
pub type LPM_SUP_R = crate::BitReader<LPM_SUP_A>;
#[doc = "LPM Supported:\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPM_SUP_A {
    #[doc = "0: LPM not supported."]
    NOT_SUPPORTED = 0,
    #[doc = "1: LPM supported."]
    SUPPORTED = 1,
}
impl From<LPM_SUP_A> for bool {
    #[inline(always)]
    fn from(variant: LPM_SUP_A) -> Self {
        variant as u8 != 0
    }
}
impl LPM_SUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPM_SUP_A {
        match self.bits {
            false => LPM_SUP_A::NOT_SUPPORTED,
            true => LPM_SUP_A::SUPPORTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_SUPPORTED`"]
    #[inline(always)]
    pub fn is_not_supported(&self) -> bool {
        *self == LPM_SUP_A::NOT_SUPPORTED
    }
    #[doc = "Checks if the value of the field is `SUPPORTED`"]
    #[inline(always)]
    pub fn is_supported(&self) -> bool {
        *self == LPM_SUP_A::SUPPORTED
    }
}
#[doc = "Field `LPM_SUP` writer - LPM Supported:"]
pub type LPM_SUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, LPM_SUP_A, O>;
impl<'a, const O: u8> LPM_SUP_W<'a, O> {
    #[doc = "LPM not supported."]
    #[inline(always)]
    pub fn not_supported(self) -> &'a mut W {
        self.variant(LPM_SUP_A::NOT_SUPPORTED)
    }
    #[doc = "LPM supported."]
    #[inline(always)]
    pub fn supported(self) -> &'a mut W {
        self.variant(LPM_SUP_A::SUPPORTED)
    }
}
#[doc = "Field `INTONNAK_AO` reader - Interrupt on NAK for interrupt and bulk OUT EP"]
pub type INTONNAK_AO_R = crate::BitReader<INTONNAK_AO_A>;
#[doc = "Interrupt on NAK for interrupt and bulk OUT EP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTONNAK_AO_A {
    #[doc = "0: Only acknowledged packets generate an interrupt"]
    ACKNOW = 0,
    #[doc = "1: Both acknowledged and NAKed packets generate interrupts."]
    ACKNOW_NAK = 1,
}
impl From<INTONNAK_AO_A> for bool {
    #[inline(always)]
    fn from(variant: INTONNAK_AO_A) -> Self {
        variant as u8 != 0
    }
}
impl INTONNAK_AO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTONNAK_AO_A {
        match self.bits {
            false => INTONNAK_AO_A::ACKNOW,
            true => INTONNAK_AO_A::ACKNOW_NAK,
        }
    }
    #[doc = "Checks if the value of the field is `ACKNOW`"]
    #[inline(always)]
    pub fn is_acknow(&self) -> bool {
        *self == INTONNAK_AO_A::ACKNOW
    }
    #[doc = "Checks if the value of the field is `ACKNOW_NAK`"]
    #[inline(always)]
    pub fn is_acknow_nak(&self) -> bool {
        *self == INTONNAK_AO_A::ACKNOW_NAK
    }
}
#[doc = "Field `INTONNAK_AO` writer - Interrupt on NAK for interrupt and bulk OUT EP"]
pub type INTONNAK_AO_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, INTONNAK_AO_A, O>;
impl<'a, const O: u8> INTONNAK_AO_W<'a, O> {
    #[doc = "Only acknowledged packets generate an interrupt"]
    #[inline(always)]
    pub fn acknow(self) -> &'a mut W {
        self.variant(INTONNAK_AO_A::ACKNOW)
    }
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    #[inline(always)]
    pub fn acknow_nak(self) -> &'a mut W {
        self.variant(INTONNAK_AO_A::ACKNOW_NAK)
    }
}
#[doc = "Field `INTONNAK_AI` reader - Interrupt on NAK for interrupt and bulk IN EP"]
pub type INTONNAK_AI_R = crate::BitReader<INTONNAK_AI_A>;
#[doc = "Interrupt on NAK for interrupt and bulk IN EP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTONNAK_AI_A {
    #[doc = "0: Only acknowledged packets generate an interrupt"]
    ACKNOW = 0,
    #[doc = "1: Both acknowledged and NAKed packets generate interrupts."]
    ACKNOW_NAK = 1,
}
impl From<INTONNAK_AI_A> for bool {
    #[inline(always)]
    fn from(variant: INTONNAK_AI_A) -> Self {
        variant as u8 != 0
    }
}
impl INTONNAK_AI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTONNAK_AI_A {
        match self.bits {
            false => INTONNAK_AI_A::ACKNOW,
            true => INTONNAK_AI_A::ACKNOW_NAK,
        }
    }
    #[doc = "Checks if the value of the field is `ACKNOW`"]
    #[inline(always)]
    pub fn is_acknow(&self) -> bool {
        *self == INTONNAK_AI_A::ACKNOW
    }
    #[doc = "Checks if the value of the field is `ACKNOW_NAK`"]
    #[inline(always)]
    pub fn is_acknow_nak(&self) -> bool {
        *self == INTONNAK_AI_A::ACKNOW_NAK
    }
}
#[doc = "Field `INTONNAK_AI` writer - Interrupt on NAK for interrupt and bulk IN EP"]
pub type INTONNAK_AI_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, INTONNAK_AI_A, O>;
impl<'a, const O: u8> INTONNAK_AI_W<'a, O> {
    #[doc = "Only acknowledged packets generate an interrupt"]
    #[inline(always)]
    pub fn acknow(self) -> &'a mut W {
        self.variant(INTONNAK_AI_A::ACKNOW)
    }
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    #[inline(always)]
    pub fn acknow_nak(self) -> &'a mut W {
        self.variant(INTONNAK_AI_A::ACKNOW_NAK)
    }
}
#[doc = "Field `INTONNAK_CO` reader - Interrupt on NAK for control OUT EP"]
pub type INTONNAK_CO_R = crate::BitReader<INTONNAK_CO_A>;
#[doc = "Interrupt on NAK for control OUT EP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTONNAK_CO_A {
    #[doc = "0: Only acknowledged packets generate an interrupt"]
    ACKNOW = 0,
    #[doc = "1: Both acknowledged and NAKed packets generate interrupts."]
    ACKNOW_NAK = 1,
}
impl From<INTONNAK_CO_A> for bool {
    #[inline(always)]
    fn from(variant: INTONNAK_CO_A) -> Self {
        variant as u8 != 0
    }
}
impl INTONNAK_CO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTONNAK_CO_A {
        match self.bits {
            false => INTONNAK_CO_A::ACKNOW,
            true => INTONNAK_CO_A::ACKNOW_NAK,
        }
    }
    #[doc = "Checks if the value of the field is `ACKNOW`"]
    #[inline(always)]
    pub fn is_acknow(&self) -> bool {
        *self == INTONNAK_CO_A::ACKNOW
    }
    #[doc = "Checks if the value of the field is `ACKNOW_NAK`"]
    #[inline(always)]
    pub fn is_acknow_nak(&self) -> bool {
        *self == INTONNAK_CO_A::ACKNOW_NAK
    }
}
#[doc = "Field `INTONNAK_CO` writer - Interrupt on NAK for control OUT EP"]
pub type INTONNAK_CO_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, INTONNAK_CO_A, O>;
impl<'a, const O: u8> INTONNAK_CO_W<'a, O> {
    #[doc = "Only acknowledged packets generate an interrupt"]
    #[inline(always)]
    pub fn acknow(self) -> &'a mut W {
        self.variant(INTONNAK_CO_A::ACKNOW)
    }
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    #[inline(always)]
    pub fn acknow_nak(self) -> &'a mut W {
        self.variant(INTONNAK_CO_A::ACKNOW_NAK)
    }
}
#[doc = "Field `INTONNAK_CI` reader - Interrupt on NAK for control IN EP"]
pub type INTONNAK_CI_R = crate::BitReader<INTONNAK_CI_A>;
#[doc = "Interrupt on NAK for control IN EP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTONNAK_CI_A {
    #[doc = "0: Only acknowledged packets generate an interrupt"]
    ACKNOW = 0,
    #[doc = "1: Both acknowledged and NAKed packets generate interrupts."]
    ACKNOW_NAK = 1,
}
impl From<INTONNAK_CI_A> for bool {
    #[inline(always)]
    fn from(variant: INTONNAK_CI_A) -> Self {
        variant as u8 != 0
    }
}
impl INTONNAK_CI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTONNAK_CI_A {
        match self.bits {
            false => INTONNAK_CI_A::ACKNOW,
            true => INTONNAK_CI_A::ACKNOW_NAK,
        }
    }
    #[doc = "Checks if the value of the field is `ACKNOW`"]
    #[inline(always)]
    pub fn is_acknow(&self) -> bool {
        *self == INTONNAK_CI_A::ACKNOW
    }
    #[doc = "Checks if the value of the field is `ACKNOW_NAK`"]
    #[inline(always)]
    pub fn is_acknow_nak(&self) -> bool {
        *self == INTONNAK_CI_A::ACKNOW_NAK
    }
}
#[doc = "Field `INTONNAK_CI` writer - Interrupt on NAK for control IN EP"]
pub type INTONNAK_CI_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, INTONNAK_CI_A, O>;
impl<'a, const O: u8> INTONNAK_CI_W<'a, O> {
    #[doc = "Only acknowledged packets generate an interrupt"]
    #[inline(always)]
    pub fn acknow(self) -> &'a mut W {
        self.variant(INTONNAK_CI_A::ACKNOW)
    }
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    #[inline(always)]
    pub fn acknow_nak(self) -> &'a mut W {
        self.variant(INTONNAK_CI_A::ACKNOW_NAK)
    }
}
#[doc = "Field `DCON` reader - Device status - connect. The connect bit must be set by SW to indicate that the device must signal a connect. The pull-up resistor on USB_DP will be enabled when this bit is set and the VbusDebounced bit is one."]
pub type DCON_R = crate::BitReader<bool>;
#[doc = "Field `DCON` writer - Device status - connect. The connect bit must be set by SW to indicate that the device must signal a connect. The pull-up resistor on USB_DP will be enabled when this bit is set and the VbusDebounced bit is one."]
pub type DCON_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, bool, O>;
#[doc = "Field `DSUS` reader - Device status - suspend. The suspend bit indicates the current suspend state. It is set to 1 when the device hasn't seen any activity on its upstream port for more than 3 milliseconds. It is reset to 0 on any activity. When the device is suspended (Suspend bit DSUS = 1) and the software writes a 0 to it, the device will generate a remote wake-up. This will only happen when the device is connected (Connect bit = 1). When the device is not connected or not suspended, a writing a 0 has no effect. Writing a 1 never has an effect."]
pub type DSUS_R = crate::BitReader<bool>;
#[doc = "Field `DSUS` writer - Device status - suspend. The suspend bit indicates the current suspend state. It is set to 1 when the device hasn't seen any activity on its upstream port for more than 3 milliseconds. It is reset to 0 on any activity. When the device is suspended (Suspend bit DSUS = 1) and the software writes a 0 to it, the device will generate a remote wake-up. This will only happen when the device is connected (Connect bit = 1). When the device is not connected or not suspended, a writing a 0 has no effect. Writing a 1 never has an effect."]
pub type DSUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, bool, O>;
#[doc = "Field `LPM_SUS` reader - Device status - LPM Suspend. This bit represents the current LPM suspend state. It is set to 1 by HW when the device has acknowledged the LPM request from the USB host and the Token Retry Time of 10us has elapsed. When the device is in the LPM suspended state (LPM suspend bit = 1) and the software writes a zero to this bit, the device will generate a remote walk-up. Software can only write a zero to this bit when the LPM_REWP bit is set to 1. HW resets this bit when it receives a host initiated resume. HW only updates the LPM_SUS bit when the LPM_SUPP bit is equal to one."]
pub type LPM_SUS_R = crate::BitReader<bool>;
#[doc = "Field `LPM_SUS` writer - Device status - LPM Suspend. This bit represents the current LPM suspend state. It is set to 1 by HW when the device has acknowledged the LPM request from the USB host and the Token Retry Time of 10us has elapsed. When the device is in the LPM suspended state (LPM suspend bit = 1) and the software writes a zero to this bit, the device will generate a remote walk-up. Software can only write a zero to this bit when the LPM_REWP bit is set to 1. HW resets this bit when it receives a host initiated resume. HW only updates the LPM_SUS bit when the LPM_SUPP bit is equal to one."]
pub type LPM_SUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, bool, O>;
#[doc = "Field `LPM_REWP` reader - LPM Remote Wake-up Enabled by USB host. HW sets this bit to one when the bRemoteWake bit in the LPM extended token is set to 1. HW will reset this bit to 0 when it receives the host initiated LPM resume, when a remote wake-up is sent by the device or when a USB bus reset is received. Software can use this bit to check if the remote wake-up feature is enabled by the host for the LPM transaction."]
pub type LPM_REWP_R = crate::BitReader<bool>;
#[doc = "Field `LPM_REWP` writer - LPM Remote Wake-up Enabled by USB host. HW sets this bit to one when the bRemoteWake bit in the LPM extended token is set to 1. HW will reset this bit to 0 when it receives the host initiated LPM resume, when a remote wake-up is sent by the device or when a USB bus reset is received. Software can use this bit to check if the remote wake-up feature is enabled by the host for the LPM transaction."]
pub type LPM_REWP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, bool, O>;
#[doc = "Field `DCON_C` reader - Device status - connect change. The Connect Change bit is set when the device's pull-up resistor is disconnected because VBus disappeared. The bit is reset by writing a one to it."]
pub type DCON_C_R = crate::BitReader<bool>;
#[doc = "Field `DCON_C` writer - Device status - connect change. The Connect Change bit is set when the device's pull-up resistor is disconnected because VBus disappeared. The bit is reset by writing a one to it."]
pub type DCON_C_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, bool, O>;
#[doc = "Field `DSUS_C` reader - Device status - suspend change. The suspend change bit is set to 1 when the suspend bit toggles. The suspend bit can toggle because: - The device goes in the suspended state - The device is disconnected - The device receives resume signaling on its upstream port. The bit is reset by writing a one to it."]
pub type DSUS_C_R = crate::BitReader<bool>;
#[doc = "Field `DSUS_C` writer - Device status - suspend change. The suspend change bit is set to 1 when the suspend bit toggles. The suspend bit can toggle because: - The device goes in the suspended state - The device is disconnected - The device receives resume signaling on its upstream port. The bit is reset by writing a one to it."]
pub type DSUS_C_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, bool, O>;
#[doc = "Field `DRES_C` reader - Device status - reset change. This bit is set when the device received a bus reset. On a bus reset the device will automatically go to the default state (unconfigured and responding to address 0). The bit is reset by writing a one to it."]
pub type DRES_C_R = crate::BitReader<bool>;
#[doc = "Field `DRES_C` writer - Device status - reset change. This bit is set when the device received a bus reset. On a bus reset the device will automatically go to the default state (unconfigured and responding to address 0). The bit is reset by writing a one to it."]
pub type DRES_C_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, bool, O>;
#[doc = "Field `VBUSDEBOUNCED` reader - This bit indicates if Vbus is detected or not. The bit raises immediately when Vbus becomes high. It drops to zero if Vbus is low for at least 3 ms. If this bit is high and the DCon bit is set, the HW will enable the pull-up resistor to signal a connect."]
pub type VBUSDEBOUNCED_R = crate::BitReader<bool>;
#[doc = "Field `VBUSDEBOUNCED` writer - This bit indicates if Vbus is detected or not. The bit raises immediately when Vbus becomes high. It drops to zero if Vbus is low for at least 3 ms. If this bit is high and the DCon bit is set, the HW will enable the pull-up resistor to signal a connect."]
pub type VBUSDEBOUNCED_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - USB device address. After bus reset, the address is reset to 0x00. If the enable bit is set, the device will respond on packets for function address DEV_ADDR. When receiving a SetAddress Control Request from the USB host, software must program the new address before completing the status phase of the SetAddress Control Request."]
    #[inline(always)]
    pub fn dev_addr(&self) -> DEV_ADDR_R {
        DEV_ADDR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - USB device enable. If this bit is set, the HW will start responding on packets for function address DEV_ADDR."]
    #[inline(always)]
    pub fn dev_en(&self) -> DEV_EN_R {
        DEV_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SETUP token received. If a SETUP token is received and acknowledged by the device, this bit is set. As long as this bit is set all received IN and OUT tokens will be NAKed by HW. SW must clear this bit by writing a one. If this bit is zero, HW will handle the tokens to the CTRL EP0 as indicated by the CTRL EP0 IN and OUT data information programmed by SW."]
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Always PLL Clock on:"]
    #[inline(always)]
    pub fn pll_on(&self) -> PLL_ON_R {
        PLL_ON_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - LPM Supported:"]
    #[inline(always)]
    pub fn lpm_sup(&self) -> LPM_SUP_R {
        LPM_SUP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt on NAK for interrupt and bulk OUT EP"]
    #[inline(always)]
    pub fn intonnak_ao(&self) -> INTONNAK_AO_R {
        INTONNAK_AO_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt on NAK for interrupt and bulk IN EP"]
    #[inline(always)]
    pub fn intonnak_ai(&self) -> INTONNAK_AI_R {
        INTONNAK_AI_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt on NAK for control OUT EP"]
    #[inline(always)]
    pub fn intonnak_co(&self) -> INTONNAK_CO_R {
        INTONNAK_CO_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt on NAK for control IN EP"]
    #[inline(always)]
    pub fn intonnak_ci(&self) -> INTONNAK_CI_R {
        INTONNAK_CI_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Device status - connect. The connect bit must be set by SW to indicate that the device must signal a connect. The pull-up resistor on USB_DP will be enabled when this bit is set and the VbusDebounced bit is one."]
    #[inline(always)]
    pub fn dcon(&self) -> DCON_R {
        DCON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Device status - suspend. The suspend bit indicates the current suspend state. It is set to 1 when the device hasn't seen any activity on its upstream port for more than 3 milliseconds. It is reset to 0 on any activity. When the device is suspended (Suspend bit DSUS = 1) and the software writes a 0 to it, the device will generate a remote wake-up. This will only happen when the device is connected (Connect bit = 1). When the device is not connected or not suspended, a writing a 0 has no effect. Writing a 1 never has an effect."]
    #[inline(always)]
    pub fn dsus(&self) -> DSUS_R {
        DSUS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Device status - LPM Suspend. This bit represents the current LPM suspend state. It is set to 1 by HW when the device has acknowledged the LPM request from the USB host and the Token Retry Time of 10us has elapsed. When the device is in the LPM suspended state (LPM suspend bit = 1) and the software writes a zero to this bit, the device will generate a remote walk-up. Software can only write a zero to this bit when the LPM_REWP bit is set to 1. HW resets this bit when it receives a host initiated resume. HW only updates the LPM_SUS bit when the LPM_SUPP bit is equal to one."]
    #[inline(always)]
    pub fn lpm_sus(&self) -> LPM_SUS_R {
        LPM_SUS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - LPM Remote Wake-up Enabled by USB host. HW sets this bit to one when the bRemoteWake bit in the LPM extended token is set to 1. HW will reset this bit to 0 when it receives the host initiated LPM resume, when a remote wake-up is sent by the device or when a USB bus reset is received. Software can use this bit to check if the remote wake-up feature is enabled by the host for the LPM transaction."]
    #[inline(always)]
    pub fn lpm_rewp(&self) -> LPM_REWP_R {
        LPM_REWP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Device status - connect change. The Connect Change bit is set when the device's pull-up resistor is disconnected because VBus disappeared. The bit is reset by writing a one to it."]
    #[inline(always)]
    pub fn dcon_c(&self) -> DCON_C_R {
        DCON_C_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Device status - suspend change. The suspend change bit is set to 1 when the suspend bit toggles. The suspend bit can toggle because: - The device goes in the suspended state - The device is disconnected - The device receives resume signaling on its upstream port. The bit is reset by writing a one to it."]
    #[inline(always)]
    pub fn dsus_c(&self) -> DSUS_C_R {
        DSUS_C_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Device status - reset change. This bit is set when the device received a bus reset. On a bus reset the device will automatically go to the default state (unconfigured and responding to address 0). The bit is reset by writing a one to it."]
    #[inline(always)]
    pub fn dres_c(&self) -> DRES_C_R {
        DRES_C_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - This bit indicates if Vbus is detected or not. The bit raises immediately when Vbus becomes high. It drops to zero if Vbus is low for at least 3 ms. If this bit is high and the DCon bit is set, the HW will enable the pull-up resistor to signal a connect."]
    #[inline(always)]
    pub fn vbusdebounced(&self) -> VBUSDEBOUNCED_R {
        VBUSDEBOUNCED_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - USB device address. After bus reset, the address is reset to 0x00. If the enable bit is set, the device will respond on packets for function address DEV_ADDR. When receiving a SetAddress Control Request from the USB host, software must program the new address before completing the status phase of the SetAddress Control Request."]
    #[inline(always)]
    #[must_use]
    pub fn dev_addr(&mut self) -> DEV_ADDR_W<0> {
        DEV_ADDR_W::new(self)
    }
    #[doc = "Bit 7 - USB device enable. If this bit is set, the HW will start responding on packets for function address DEV_ADDR."]
    #[inline(always)]
    #[must_use]
    pub fn dev_en(&mut self) -> DEV_EN_W<7> {
        DEV_EN_W::new(self)
    }
    #[doc = "Bit 8 - SETUP token received. If a SETUP token is received and acknowledged by the device, this bit is set. As long as this bit is set all received IN and OUT tokens will be NAKed by HW. SW must clear this bit by writing a one. If this bit is zero, HW will handle the tokens to the CTRL EP0 as indicated by the CTRL EP0 IN and OUT data information programmed by SW."]
    #[inline(always)]
    #[must_use]
    pub fn setup(&mut self) -> SETUP_W<8> {
        SETUP_W::new(self)
    }
    #[doc = "Bit 9 - Always PLL Clock on:"]
    #[inline(always)]
    #[must_use]
    pub fn pll_on(&mut self) -> PLL_ON_W<9> {
        PLL_ON_W::new(self)
    }
    #[doc = "Bit 11 - LPM Supported:"]
    #[inline(always)]
    #[must_use]
    pub fn lpm_sup(&mut self) -> LPM_SUP_W<11> {
        LPM_SUP_W::new(self)
    }
    #[doc = "Bit 12 - Interrupt on NAK for interrupt and bulk OUT EP"]
    #[inline(always)]
    #[must_use]
    pub fn intonnak_ao(&mut self) -> INTONNAK_AO_W<12> {
        INTONNAK_AO_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt on NAK for interrupt and bulk IN EP"]
    #[inline(always)]
    #[must_use]
    pub fn intonnak_ai(&mut self) -> INTONNAK_AI_W<13> {
        INTONNAK_AI_W::new(self)
    }
    #[doc = "Bit 14 - Interrupt on NAK for control OUT EP"]
    #[inline(always)]
    #[must_use]
    pub fn intonnak_co(&mut self) -> INTONNAK_CO_W<14> {
        INTONNAK_CO_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt on NAK for control IN EP"]
    #[inline(always)]
    #[must_use]
    pub fn intonnak_ci(&mut self) -> INTONNAK_CI_W<15> {
        INTONNAK_CI_W::new(self)
    }
    #[doc = "Bit 16 - Device status - connect. The connect bit must be set by SW to indicate that the device must signal a connect. The pull-up resistor on USB_DP will be enabled when this bit is set and the VbusDebounced bit is one."]
    #[inline(always)]
    #[must_use]
    pub fn dcon(&mut self) -> DCON_W<16> {
        DCON_W::new(self)
    }
    #[doc = "Bit 17 - Device status - suspend. The suspend bit indicates the current suspend state. It is set to 1 when the device hasn't seen any activity on its upstream port for more than 3 milliseconds. It is reset to 0 on any activity. When the device is suspended (Suspend bit DSUS = 1) and the software writes a 0 to it, the device will generate a remote wake-up. This will only happen when the device is connected (Connect bit = 1). When the device is not connected or not suspended, a writing a 0 has no effect. Writing a 1 never has an effect."]
    #[inline(always)]
    #[must_use]
    pub fn dsus(&mut self) -> DSUS_W<17> {
        DSUS_W::new(self)
    }
    #[doc = "Bit 19 - Device status - LPM Suspend. This bit represents the current LPM suspend state. It is set to 1 by HW when the device has acknowledged the LPM request from the USB host and the Token Retry Time of 10us has elapsed. When the device is in the LPM suspended state (LPM suspend bit = 1) and the software writes a zero to this bit, the device will generate a remote walk-up. Software can only write a zero to this bit when the LPM_REWP bit is set to 1. HW resets this bit when it receives a host initiated resume. HW only updates the LPM_SUS bit when the LPM_SUPP bit is equal to one."]
    #[inline(always)]
    #[must_use]
    pub fn lpm_sus(&mut self) -> LPM_SUS_W<19> {
        LPM_SUS_W::new(self)
    }
    #[doc = "Bit 20 - LPM Remote Wake-up Enabled by USB host. HW sets this bit to one when the bRemoteWake bit in the LPM extended token is set to 1. HW will reset this bit to 0 when it receives the host initiated LPM resume, when a remote wake-up is sent by the device or when a USB bus reset is received. Software can use this bit to check if the remote wake-up feature is enabled by the host for the LPM transaction."]
    #[inline(always)]
    #[must_use]
    pub fn lpm_rewp(&mut self) -> LPM_REWP_W<20> {
        LPM_REWP_W::new(self)
    }
    #[doc = "Bit 24 - Device status - connect change. The Connect Change bit is set when the device's pull-up resistor is disconnected because VBus disappeared. The bit is reset by writing a one to it."]
    #[inline(always)]
    #[must_use]
    pub fn dcon_c(&mut self) -> DCON_C_W<24> {
        DCON_C_W::new(self)
    }
    #[doc = "Bit 25 - Device status - suspend change. The suspend change bit is set to 1 when the suspend bit toggles. The suspend bit can toggle because: - The device goes in the suspended state - The device is disconnected - The device receives resume signaling on its upstream port. The bit is reset by writing a one to it."]
    #[inline(always)]
    #[must_use]
    pub fn dsus_c(&mut self) -> DSUS_C_W<25> {
        DSUS_C_W::new(self)
    }
    #[doc = "Bit 26 - Device status - reset change. This bit is set when the device received a bus reset. On a bus reset the device will automatically go to the default state (unconfigured and responding to address 0). The bit is reset by writing a one to it."]
    #[inline(always)]
    #[must_use]
    pub fn dres_c(&mut self) -> DRES_C_W<26> {
        DRES_C_W::new(self)
    }
    #[doc = "Bit 28 - This bit indicates if Vbus is detected or not. The bit raises immediately when Vbus becomes high. It drops to zero if Vbus is low for at least 3 ms. If this bit is high and the DCon bit is set, the HW will enable the pull-up resistor to signal a connect."]
    #[inline(always)]
    #[must_use]
    pub fn vbusdebounced(&mut self) -> VBUSDEBOUNCED_W<28> {
        VBUSDEBOUNCED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Device Command/Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devcmdstat](index.html) module"]
pub struct DEVCMDSTAT_SPEC;
impl crate::RegisterSpec for DEVCMDSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devcmdstat::R](R) reader structure"]
impl crate::Readable for DEVCMDSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [devcmdstat::W](W) writer structure"]
impl crate::Writable for DEVCMDSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEVCMDSTAT to value 0x0800"]
impl crate::Resettable for DEVCMDSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0800;
}
