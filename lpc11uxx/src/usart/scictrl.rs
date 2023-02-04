#[doc = "Register `SCICTRL` reader"]
pub struct R(crate::R<SCICTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCICTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCICTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCICTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCICTRL` writer"]
pub struct W(crate::W<SCICTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCICTRL_SPEC>;
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
impl From<crate::W<SCICTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCICTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCIEN` reader - Smart Card Interface Enable."]
pub type SCIEN_R = crate::BitReader<SCIEN_A>;
#[doc = "Smart Card Interface Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCIEN_A {
    #[doc = "0: Smart card interface disabled."]
    DISABLED = 0,
    #[doc = "1: Asynchronous half duplex smart card interface is enabled."]
    ENABLED = 1,
}
impl From<SCIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SCIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SCIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCIEN_A {
        match self.bits {
            false => SCIEN_A::DISABLED,
            true => SCIEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SCIEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SCIEN_A::ENABLED
    }
}
#[doc = "Field `SCIEN` writer - Smart Card Interface Enable."]
pub type SCIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCICTRL_SPEC, SCIEN_A, O>;
impl<'a, const O: u8> SCIEN_W<'a, O> {
    #[doc = "Smart card interface disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SCIEN_A::DISABLED)
    }
    #[doc = "Asynchronous half duplex smart card interface is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SCIEN_A::ENABLED)
    }
}
#[doc = "Field `NACKDIS` reader - NACK response disable. Only applicable in T=0."]
pub type NACKDIS_R = crate::BitReader<NACKDIS_A>;
#[doc = "NACK response disable. Only applicable in T=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKDIS_A {
    #[doc = "0: A NACK response is enabled."]
    ENABLED = 0,
    #[doc = "1: A NACK response is inhibited."]
    DISABLED = 1,
}
impl From<NACKDIS_A> for bool {
    #[inline(always)]
    fn from(variant: NACKDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl NACKDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NACKDIS_A {
        match self.bits {
            false => NACKDIS_A::ENABLED,
            true => NACKDIS_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NACKDIS_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NACKDIS_A::DISABLED
    }
}
#[doc = "Field `NACKDIS` writer - NACK response disable. Only applicable in T=0."]
pub type NACKDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCICTRL_SPEC, NACKDIS_A, O>;
impl<'a, const O: u8> NACKDIS_W<'a, O> {
    #[doc = "A NACK response is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NACKDIS_A::ENABLED)
    }
    #[doc = "A NACK response is inhibited."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NACKDIS_A::DISABLED)
    }
}
#[doc = "Field `PROTSEL` reader - Protocol selection as defined in the ISO7816-3 standard."]
pub type PROTSEL_R = crate::BitReader<PROTSEL_A>;
#[doc = "Protocol selection as defined in the ISO7816-3 standard.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROTSEL_A {
    #[doc = "0: T = 0"]
    T_EQ_0 = 0,
    #[doc = "1: T = 1"]
    T_EQ_1 = 1,
}
impl From<PROTSEL_A> for bool {
    #[inline(always)]
    fn from(variant: PROTSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl PROTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROTSEL_A {
        match self.bits {
            false => PROTSEL_A::T_EQ_0,
            true => PROTSEL_A::T_EQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `T_EQ_0`"]
    #[inline(always)]
    pub fn is_t_eq_0(&self) -> bool {
        *self == PROTSEL_A::T_EQ_0
    }
    #[doc = "Checks if the value of the field is `T_EQ_1`"]
    #[inline(always)]
    pub fn is_t_eq_1(&self) -> bool {
        *self == PROTSEL_A::T_EQ_1
    }
}
#[doc = "Field `PROTSEL` writer - Protocol selection as defined in the ISO7816-3 standard."]
pub type PROTSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCICTRL_SPEC, PROTSEL_A, O>;
impl<'a, const O: u8> PROTSEL_W<'a, O> {
    #[doc = "T = 0"]
    #[inline(always)]
    pub fn t_eq_0(self) -> &'a mut W {
        self.variant(PROTSEL_A::T_EQ_0)
    }
    #[doc = "T = 1"]
    #[inline(always)]
    pub fn t_eq_1(self) -> &'a mut W {
        self.variant(PROTSEL_A::T_EQ_1)
    }
}
#[doc = "Field `TXRETRY` reader - When the protocol selection T bit (above) is 0, the field controls the maximum number of retransmissions that the USART will attempt if the remote device signals NACK. When NACK has occurred this number of times plus one, the Tx Error bit in the LSR is set, an interrupt is requested if enabled, and the USART is locked until the FIFO is cleared."]
pub type TXRETRY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXRETRY` writer - When the protocol selection T bit (above) is 0, the field controls the maximum number of retransmissions that the USART will attempt if the remote device signals NACK. When NACK has occurred this number of times plus one, the Tx Error bit in the LSR is set, an interrupt is requested if enabled, and the USART is locked until the FIFO is cleared."]
pub type TXRETRY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCICTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `XTRAGUARD` reader - When the protocol selection T bit (above) is 0, this field indicates the number of bit times (ETUs) by which the guard time after a character transmitted by the USART should exceed the nominal 2 bit times. 0xFF in this field may indicate that there is just a single bit after a character and 11 bit times/character"]
pub type XTRAGUARD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XTRAGUARD` writer - When the protocol selection T bit (above) is 0, this field indicates the number of bit times (ETUs) by which the guard time after a character transmitted by the USART should exceed the nominal 2 bit times. 0xFF in this field may indicate that there is just a single bit after a character and 11 bit times/character"]
pub type XTRAGUARD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCICTRL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Smart Card Interface Enable."]
    #[inline(always)]
    pub fn scien(&self) -> SCIEN_R {
        SCIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NACK response disable. Only applicable in T=0."]
    #[inline(always)]
    pub fn nackdis(&self) -> NACKDIS_R {
        NACKDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Protocol selection as defined in the ISO7816-3 standard."]
    #[inline(always)]
    pub fn protsel(&self) -> PROTSEL_R {
        PROTSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 5:7 - When the protocol selection T bit (above) is 0, the field controls the maximum number of retransmissions that the USART will attempt if the remote device signals NACK. When NACK has occurred this number of times plus one, the Tx Error bit in the LSR is set, an interrupt is requested if enabled, and the USART is locked until the FIFO is cleared."]
    #[inline(always)]
    pub fn txretry(&self) -> TXRETRY_R {
        TXRETRY_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:15 - When the protocol selection T bit (above) is 0, this field indicates the number of bit times (ETUs) by which the guard time after a character transmitted by the USART should exceed the nominal 2 bit times. 0xFF in this field may indicate that there is just a single bit after a character and 11 bit times/character"]
    #[inline(always)]
    pub fn xtraguard(&self) -> XTRAGUARD_R {
        XTRAGUARD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Smart Card Interface Enable."]
    #[inline(always)]
    #[must_use]
    pub fn scien(&mut self) -> SCIEN_W<0> {
        SCIEN_W::new(self)
    }
    #[doc = "Bit 1 - NACK response disable. Only applicable in T=0."]
    #[inline(always)]
    #[must_use]
    pub fn nackdis(&mut self) -> NACKDIS_W<1> {
        NACKDIS_W::new(self)
    }
    #[doc = "Bit 2 - Protocol selection as defined in the ISO7816-3 standard."]
    #[inline(always)]
    #[must_use]
    pub fn protsel(&mut self) -> PROTSEL_W<2> {
        PROTSEL_W::new(self)
    }
    #[doc = "Bits 5:7 - When the protocol selection T bit (above) is 0, the field controls the maximum number of retransmissions that the USART will attempt if the remote device signals NACK. When NACK has occurred this number of times plus one, the Tx Error bit in the LSR is set, an interrupt is requested if enabled, and the USART is locked until the FIFO is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn txretry(&mut self) -> TXRETRY_W<5> {
        TXRETRY_W::new(self)
    }
    #[doc = "Bits 8:15 - When the protocol selection T bit (above) is 0, this field indicates the number of bit times (ETUs) by which the guard time after a character transmitted by the USART should exceed the nominal 2 bit times. 0xFF in this field may indicate that there is just a single bit after a character and 11 bit times/character"]
    #[inline(always)]
    #[must_use]
    pub fn xtraguard(&mut self) -> XTRAGUARD_W<8> {
        XTRAGUARD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Smart Card Interface Control register. Enables and configures the Smart Card Interface feature.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scictrl](index.html) module"]
pub struct SCICTRL_SPEC;
impl crate::RegisterSpec for SCICTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scictrl::R](R) reader structure"]
impl crate::Readable for SCICTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scictrl::W](W) writer structure"]
impl crate::Writable for SCICTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCICTRL to value 0"]
impl crate::Resettable for SCICTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
