#[doc = "Register `ACR` reader"]
pub struct R(crate::R<ACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACR` writer"]
pub struct W(crate::W<ACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACR_SPEC>;
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
impl From<crate::W<ACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - This bit is automatically cleared after auto-baud completion."]
pub type START_R = crate::BitReader<START_A>;
#[doc = "This bit is automatically cleared after auto-baud completion.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum START_A {
    #[doc = "0: Auto-baud stop (auto-baud is not running)."]
    STOPPED = 0,
    #[doc = "1: Auto-baud start (auto-baud is running). Auto-baud run bit. This bit is automatically cleared after auto-baud completion."]
    RUNNING = 1,
}
impl From<START_A> for bool {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as u8 != 0
    }
}
impl START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> START_A {
        match self.bits {
            false => START_A::STOPPED,
            true => START_A::RUNNING,
        }
    }
    #[doc = "Checks if the value of the field is `STOPPED`"]
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == START_A::STOPPED
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == START_A::RUNNING
    }
}
#[doc = "Field `START` writer - This bit is automatically cleared after auto-baud completion."]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, START_A, O>;
impl<'a, const O: u8> START_W<'a, O> {
    #[doc = "Auto-baud stop (auto-baud is not running)."]
    #[inline(always)]
    pub fn stopped(self) -> &'a mut W {
        self.variant(START_A::STOPPED)
    }
    #[doc = "Auto-baud start (auto-baud is running). Auto-baud run bit. This bit is automatically cleared after auto-baud completion."]
    #[inline(always)]
    pub fn running(self) -> &'a mut W {
        self.variant(START_A::RUNNING)
    }
}
#[doc = "Field `MODE` reader - Auto-baud mode select bit."]
pub type MODE_R = crate::BitReader<MODE_A>;
#[doc = "Auto-baud mode select bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODE_A {
    #[doc = "0: Mode 0."]
    MODE0 = 0,
    #[doc = "1: Mode 1."]
    MODE1 = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::MODE0,
            true => MODE_A::MODE1,
        }
    }
    #[doc = "Checks if the value of the field is `MODE0`"]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == MODE_A::MODE0
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == MODE_A::MODE1
    }
}
#[doc = "Field `MODE` writer - Auto-baud mode select bit."]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, MODE_A, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Mode 0."]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut W {
        self.variant(MODE_A::MODE0)
    }
    #[doc = "Mode 1."]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(MODE_A::MODE1)
    }
}
#[doc = "Field `AUTORESTART` reader - Start mode"]
pub type AUTORESTART_R = crate::BitReader<AUTORESTART_A>;
#[doc = "Start mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTORESTART_A {
    #[doc = "0: No restart"]
    NO_RESTART = 0,
    #[doc = "1: Restart in case of time-out (counter restarts at next USART Rx falling edge)"]
    RESTART_ON_TIMEOUT = 1,
}
impl From<AUTORESTART_A> for bool {
    #[inline(always)]
    fn from(variant: AUTORESTART_A) -> Self {
        variant as u8 != 0
    }
}
impl AUTORESTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTORESTART_A {
        match self.bits {
            false => AUTORESTART_A::NO_RESTART,
            true => AUTORESTART_A::RESTART_ON_TIMEOUT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_RESTART`"]
    #[inline(always)]
    pub fn is_no_restart(&self) -> bool {
        *self == AUTORESTART_A::NO_RESTART
    }
    #[doc = "Checks if the value of the field is `RESTART_ON_TIMEOUT`"]
    #[inline(always)]
    pub fn is_restart_on_timeout(&self) -> bool {
        *self == AUTORESTART_A::RESTART_ON_TIMEOUT
    }
}
#[doc = "Field `AUTORESTART` writer - Start mode"]
pub type AUTORESTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, AUTORESTART_A, O>;
impl<'a, const O: u8> AUTORESTART_W<'a, O> {
    #[doc = "No restart"]
    #[inline(always)]
    pub fn no_restart(self) -> &'a mut W {
        self.variant(AUTORESTART_A::NO_RESTART)
    }
    #[doc = "Restart in case of time-out (counter restarts at next USART Rx falling edge)"]
    #[inline(always)]
    pub fn restart_on_timeout(self) -> &'a mut W {
        self.variant(AUTORESTART_A::RESTART_ON_TIMEOUT)
    }
}
#[doc = "Field `ABEOINTCLR` reader - End of auto-baud interrupt clear bit (write only accessible)."]
pub type ABEOINTCLR_R = crate::BitReader<ABEOINTCLR_A>;
#[doc = "End of auto-baud interrupt clear bit (write only accessible).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABEOINTCLR_A {
    #[doc = "0: Writing a 0 has no impact."]
    NO_IMPACT = 0,
    #[doc = "1: Writing a 1 will clear the corresponding interrupt in the IIR."]
    CLEAR = 1,
}
impl From<ABEOINTCLR_A> for bool {
    #[inline(always)]
    fn from(variant: ABEOINTCLR_A) -> Self {
        variant as u8 != 0
    }
}
impl ABEOINTCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABEOINTCLR_A {
        match self.bits {
            false => ABEOINTCLR_A::NO_IMPACT,
            true => ABEOINTCLR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_IMPACT`"]
    #[inline(always)]
    pub fn is_no_impact(&self) -> bool {
        *self == ABEOINTCLR_A::NO_IMPACT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == ABEOINTCLR_A::CLEAR
    }
}
#[doc = "Field `ABEOINTCLR` writer - End of auto-baud interrupt clear bit (write only accessible)."]
pub type ABEOINTCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, ABEOINTCLR_A, O>;
impl<'a, const O: u8> ABEOINTCLR_W<'a, O> {
    #[doc = "Writing a 0 has no impact."]
    #[inline(always)]
    pub fn no_impact(self) -> &'a mut W {
        self.variant(ABEOINTCLR_A::NO_IMPACT)
    }
    #[doc = "Writing a 1 will clear the corresponding interrupt in the IIR."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ABEOINTCLR_A::CLEAR)
    }
}
#[doc = "Field `ABTOINTCLR` reader - Auto-baud time-out interrupt clear bit (write only accessible)."]
pub type ABTOINTCLR_R = crate::BitReader<ABTOINTCLR_A>;
#[doc = "Auto-baud time-out interrupt clear bit (write only accessible).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABTOINTCLR_A {
    #[doc = "0: Writing a 0 has no impact."]
    NO_IMPACT = 0,
    #[doc = "1: Writing a 1 will clear the corresponding interrupt in the IIR."]
    CLEAR = 1,
}
impl From<ABTOINTCLR_A> for bool {
    #[inline(always)]
    fn from(variant: ABTOINTCLR_A) -> Self {
        variant as u8 != 0
    }
}
impl ABTOINTCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABTOINTCLR_A {
        match self.bits {
            false => ABTOINTCLR_A::NO_IMPACT,
            true => ABTOINTCLR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_IMPACT`"]
    #[inline(always)]
    pub fn is_no_impact(&self) -> bool {
        *self == ABTOINTCLR_A::NO_IMPACT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == ABTOINTCLR_A::CLEAR
    }
}
#[doc = "Field `ABTOINTCLR` writer - Auto-baud time-out interrupt clear bit (write only accessible)."]
pub type ABTOINTCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, ABTOINTCLR_A, O>;
impl<'a, const O: u8> ABTOINTCLR_W<'a, O> {
    #[doc = "Writing a 0 has no impact."]
    #[inline(always)]
    pub fn no_impact(self) -> &'a mut W {
        self.variant(ABTOINTCLR_A::NO_IMPACT)
    }
    #[doc = "Writing a 1 will clear the corresponding interrupt in the IIR."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ABTOINTCLR_A::CLEAR)
    }
}
impl R {
    #[doc = "Bit 0 - This bit is automatically cleared after auto-baud completion."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Auto-baud mode select bit."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Start mode"]
    #[inline(always)]
    pub fn autorestart(&self) -> AUTORESTART_R {
        AUTORESTART_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - End of auto-baud interrupt clear bit (write only accessible)."]
    #[inline(always)]
    pub fn abeointclr(&self) -> ABEOINTCLR_R {
        ABEOINTCLR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Auto-baud time-out interrupt clear bit (write only accessible)."]
    #[inline(always)]
    pub fn abtointclr(&self) -> ABTOINTCLR_R {
        ABTOINTCLR_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is automatically cleared after auto-baud completion."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Bit 1 - Auto-baud mode select bit."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<1> {
        MODE_W::new(self)
    }
    #[doc = "Bit 2 - Start mode"]
    #[inline(always)]
    #[must_use]
    pub fn autorestart(&mut self) -> AUTORESTART_W<2> {
        AUTORESTART_W::new(self)
    }
    #[doc = "Bit 8 - End of auto-baud interrupt clear bit (write only accessible)."]
    #[inline(always)]
    #[must_use]
    pub fn abeointclr(&mut self) -> ABEOINTCLR_W<8> {
        ABEOINTCLR_W::new(self)
    }
    #[doc = "Bit 9 - Auto-baud time-out interrupt clear bit (write only accessible)."]
    #[inline(always)]
    #[must_use]
    pub fn abtointclr(&mut self) -> ABTOINTCLR_W<9> {
        ABTOINTCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auto-baud Control Register. Contains controls for the auto-baud feature.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acr](index.html) module"]
pub struct ACR_SPEC;
impl crate::RegisterSpec for ACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acr::R](R) reader structure"]
impl crate::Readable for ACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acr::W](W) writer structure"]
impl crate::Writable for ACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACR to value 0"]
impl crate::Resettable for ACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
