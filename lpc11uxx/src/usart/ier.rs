#[doc = "Register `IER` reader"]
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RBRINTEN` reader - RBR Interrupt Enable. Enables the Receive Data Available interrupt. It also controls the Character Receive Time-out interrupt."]
pub type RBRINTEN_R = crate::BitReader<RBRINTEN_A>;
#[doc = "RBR Interrupt Enable. Enables the Receive Data Available interrupt. It also controls the Character Receive Time-out interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RBRINTEN_A {
    #[doc = "0: Disable the RDA interrupt."]
    DISABLE = 0,
    #[doc = "1: Enable the RDA interrupt."]
    ENABLE = 1,
}
impl From<RBRINTEN_A> for bool {
    #[inline(always)]
    fn from(variant: RBRINTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RBRINTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBRINTEN_A {
        match self.bits {
            false => RBRINTEN_A::DISABLE,
            true => RBRINTEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RBRINTEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RBRINTEN_A::ENABLE
    }
}
#[doc = "Field `RBRINTEN` writer - RBR Interrupt Enable. Enables the Receive Data Available interrupt. It also controls the Character Receive Time-out interrupt."]
pub type RBRINTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, RBRINTEN_A, O>;
impl<'a, const O: u8> RBRINTEN_W<'a, O> {
    #[doc = "Disable the RDA interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RBRINTEN_A::DISABLE)
    }
    #[doc = "Enable the RDA interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RBRINTEN_A::ENABLE)
    }
}
#[doc = "Field `THREINTEN` reader - THRE Interrupt Enable. Enables the THRE interrupt. The status of this interrupt can be read from LSR\\[5\\]."]
pub type THREINTEN_R = crate::BitReader<THREINTEN_A>;
#[doc = "THRE Interrupt Enable. Enables the THRE interrupt. The status of this interrupt can be read from LSR\\[5\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum THREINTEN_A {
    #[doc = "0: Disable the THRE interrupt."]
    DISABLE = 0,
    #[doc = "1: Enable the THRE interrupt."]
    ENABLE = 1,
}
impl From<THREINTEN_A> for bool {
    #[inline(always)]
    fn from(variant: THREINTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl THREINTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THREINTEN_A {
        match self.bits {
            false => THREINTEN_A::DISABLE,
            true => THREINTEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == THREINTEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == THREINTEN_A::ENABLE
    }
}
#[doc = "Field `THREINTEN` writer - THRE Interrupt Enable. Enables the THRE interrupt. The status of this interrupt can be read from LSR\\[5\\]."]
pub type THREINTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, THREINTEN_A, O>;
impl<'a, const O: u8> THREINTEN_W<'a, O> {
    #[doc = "Disable the THRE interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(THREINTEN_A::DISABLE)
    }
    #[doc = "Enable the THRE interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(THREINTEN_A::ENABLE)
    }
}
#[doc = "Field `RLSINTEN` reader - Enables the Receive Line Status interrupt. The status of this interrupt can be read from LSR\\[4:1\\]."]
pub type RLSINTEN_R = crate::BitReader<RLSINTEN_A>;
#[doc = "Enables the Receive Line Status interrupt. The status of this interrupt can be read from LSR\\[4:1\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RLSINTEN_A {
    #[doc = "0: Disable the RLS interrupt."]
    DISABLE = 0,
    #[doc = "1: Enable the RLS interrupt."]
    ENABLE = 1,
}
impl From<RLSINTEN_A> for bool {
    #[inline(always)]
    fn from(variant: RLSINTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RLSINTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RLSINTEN_A {
        match self.bits {
            false => RLSINTEN_A::DISABLE,
            true => RLSINTEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RLSINTEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RLSINTEN_A::ENABLE
    }
}
#[doc = "Field `RLSINTEN` writer - Enables the Receive Line Status interrupt. The status of this interrupt can be read from LSR\\[4:1\\]."]
pub type RLSINTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, RLSINTEN_A, O>;
impl<'a, const O: u8> RLSINTEN_W<'a, O> {
    #[doc = "Disable the RLS interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RLSINTEN_A::DISABLE)
    }
    #[doc = "Enable the RLS interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RLSINTEN_A::ENABLE)
    }
}
#[doc = "Field `MSINTEN` reader - Enables the Modem Status interrupt. The components of this interrupt can be read from the MSR."]
pub type MSINTEN_R = crate::BitReader<MSINTEN_A>;
#[doc = "Enables the Modem Status interrupt. The components of this interrupt can be read from the MSR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSINTEN_A {
    #[doc = "0: Disable the MS interrupt."]
    DISABLE = 0,
    #[doc = "1: Enable the MS interrupt."]
    ENABLE = 1,
}
impl From<MSINTEN_A> for bool {
    #[inline(always)]
    fn from(variant: MSINTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MSINTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSINTEN_A {
        match self.bits {
            false => MSINTEN_A::DISABLE,
            true => MSINTEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MSINTEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MSINTEN_A::ENABLE
    }
}
#[doc = "Field `MSINTEN` writer - Enables the Modem Status interrupt. The components of this interrupt can be read from the MSR."]
pub type MSINTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, MSINTEN_A, O>;
impl<'a, const O: u8> MSINTEN_W<'a, O> {
    #[doc = "Disable the MS interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MSINTEN_A::DISABLE)
    }
    #[doc = "Enable the MS interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MSINTEN_A::ENABLE)
    }
}
#[doc = "Field `ABEOINTEN` reader - Enables the end of auto-baud interrupt."]
pub type ABEOINTEN_R = crate::BitReader<ABEOINTEN_A>;
#[doc = "Enables the end of auto-baud interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABEOINTEN_A {
    #[doc = "0: Disable end of auto-baud Interrupt."]
    DISABLED = 0,
    #[doc = "1: Enable end of auto-baud Interrupt."]
    ENABLED = 1,
}
impl From<ABEOINTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ABEOINTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ABEOINTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABEOINTEN_A {
        match self.bits {
            false => ABEOINTEN_A::DISABLED,
            true => ABEOINTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ABEOINTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ABEOINTEN_A::ENABLED
    }
}
#[doc = "Field `ABEOINTEN` writer - Enables the end of auto-baud interrupt."]
pub type ABEOINTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, ABEOINTEN_A, O>;
impl<'a, const O: u8> ABEOINTEN_W<'a, O> {
    #[doc = "Disable end of auto-baud Interrupt."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ABEOINTEN_A::DISABLED)
    }
    #[doc = "Enable end of auto-baud Interrupt."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ABEOINTEN_A::ENABLED)
    }
}
#[doc = "Field `ABTOINTEN` reader - Enables the auto-baud time-out interrupt."]
pub type ABTOINTEN_R = crate::BitReader<ABTOINTEN_A>;
#[doc = "Enables the auto-baud time-out interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABTOINTEN_A {
    #[doc = "0: Disable auto-baud time-out Interrupt."]
    DISABLED = 0,
    #[doc = "1: Enable auto-baud time-out Interrupt."]
    ENABLED = 1,
}
impl From<ABTOINTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ABTOINTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ABTOINTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABTOINTEN_A {
        match self.bits {
            false => ABTOINTEN_A::DISABLED,
            true => ABTOINTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ABTOINTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ABTOINTEN_A::ENABLED
    }
}
#[doc = "Field `ABTOINTEN` writer - Enables the auto-baud time-out interrupt."]
pub type ABTOINTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, ABTOINTEN_A, O>;
impl<'a, const O: u8> ABTOINTEN_W<'a, O> {
    #[doc = "Disable auto-baud time-out Interrupt."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ABTOINTEN_A::DISABLED)
    }
    #[doc = "Enable auto-baud time-out Interrupt."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ABTOINTEN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - RBR Interrupt Enable. Enables the Receive Data Available interrupt. It also controls the Character Receive Time-out interrupt."]
    #[inline(always)]
    pub fn rbrinten(&self) -> RBRINTEN_R {
        RBRINTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - THRE Interrupt Enable. Enables the THRE interrupt. The status of this interrupt can be read from LSR\\[5\\]."]
    #[inline(always)]
    pub fn threinten(&self) -> THREINTEN_R {
        THREINTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables the Receive Line Status interrupt. The status of this interrupt can be read from LSR\\[4:1\\]."]
    #[inline(always)]
    pub fn rlsinten(&self) -> RLSINTEN_R {
        RLSINTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables the Modem Status interrupt. The components of this interrupt can be read from the MSR."]
    #[inline(always)]
    pub fn msinten(&self) -> MSINTEN_R {
        MSINTEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Enables the end of auto-baud interrupt."]
    #[inline(always)]
    pub fn abeointen(&self) -> ABEOINTEN_R {
        ABEOINTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enables the auto-baud time-out interrupt."]
    #[inline(always)]
    pub fn abtointen(&self) -> ABTOINTEN_R {
        ABTOINTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RBR Interrupt Enable. Enables the Receive Data Available interrupt. It also controls the Character Receive Time-out interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rbrinten(&mut self) -> RBRINTEN_W<0> {
        RBRINTEN_W::new(self)
    }
    #[doc = "Bit 1 - THRE Interrupt Enable. Enables the THRE interrupt. The status of this interrupt can be read from LSR\\[5\\]."]
    #[inline(always)]
    #[must_use]
    pub fn threinten(&mut self) -> THREINTEN_W<1> {
        THREINTEN_W::new(self)
    }
    #[doc = "Bit 2 - Enables the Receive Line Status interrupt. The status of this interrupt can be read from LSR\\[4:1\\]."]
    #[inline(always)]
    #[must_use]
    pub fn rlsinten(&mut self) -> RLSINTEN_W<2> {
        RLSINTEN_W::new(self)
    }
    #[doc = "Bit 3 - Enables the Modem Status interrupt. The components of this interrupt can be read from the MSR."]
    #[inline(always)]
    #[must_use]
    pub fn msinten(&mut self) -> MSINTEN_W<3> {
        MSINTEN_W::new(self)
    }
    #[doc = "Bit 8 - Enables the end of auto-baud interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn abeointen(&mut self) -> ABEOINTEN_W<8> {
        ABEOINTEN_W::new(self)
    }
    #[doc = "Bit 9 - Enables the auto-baud time-out interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn abtointen(&mut self) -> ABTOINTEN_W<9> {
        ABTOINTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential USART interrupts. (DLAB=0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier::R](R) reader structure"]
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
