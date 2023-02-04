#[doc = "Register `LCR` reader"]
pub struct R(crate::R<LCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCR` writer"]
pub struct W(crate::W<LCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCR_SPEC>;
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
impl From<crate::W<LCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WLS` reader - Word Length Select"]
pub type WLS_R = crate::FieldReader<u8, WLS_A>;
#[doc = "Word Length Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WLS_A {
    #[doc = "0: 5-bit character length."]
    _5_BIT = 0,
    #[doc = "1: 6-bit character length."]
    _6_BIT = 1,
    #[doc = "2: 7-bit character length."]
    _7_BIT = 2,
    #[doc = "3: 8-bit character length."]
    _8_BIT = 3,
}
impl From<WLS_A> for u8 {
    #[inline(always)]
    fn from(variant: WLS_A) -> Self {
        variant as _
    }
}
impl WLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WLS_A {
        match self.bits {
            0 => WLS_A::_5_BIT,
            1 => WLS_A::_6_BIT,
            2 => WLS_A::_7_BIT,
            3 => WLS_A::_8_BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_5_BIT`"]
    #[inline(always)]
    pub fn is_5_bit(&self) -> bool {
        *self == WLS_A::_5_BIT
    }
    #[doc = "Checks if the value of the field is `_6_BIT`"]
    #[inline(always)]
    pub fn is_6_bit(&self) -> bool {
        *self == WLS_A::_6_BIT
    }
    #[doc = "Checks if the value of the field is `_7_BIT`"]
    #[inline(always)]
    pub fn is_7_bit(&self) -> bool {
        *self == WLS_A::_7_BIT
    }
    #[doc = "Checks if the value of the field is `_8_BIT`"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == WLS_A::_8_BIT
    }
}
#[doc = "Field `WLS` writer - Word Length Select"]
pub type WLS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, LCR_SPEC, u8, WLS_A, 2, O>;
impl<'a, const O: u8> WLS_W<'a, O> {
    #[doc = "5-bit character length."]
    #[inline(always)]
    pub fn _5_bit(self) -> &'a mut W {
        self.variant(WLS_A::_5_BIT)
    }
    #[doc = "6-bit character length."]
    #[inline(always)]
    pub fn _6_bit(self) -> &'a mut W {
        self.variant(WLS_A::_6_BIT)
    }
    #[doc = "7-bit character length."]
    #[inline(always)]
    pub fn _7_bit(self) -> &'a mut W {
        self.variant(WLS_A::_7_BIT)
    }
    #[doc = "8-bit character length."]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut W {
        self.variant(WLS_A::_8_BIT)
    }
}
#[doc = "Field `SBS` reader - Stop Bit Select"]
pub type SBS_R = crate::BitReader<SBS_A>;
#[doc = "Stop Bit Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBS_A {
    #[doc = "0: 1 stop bit."]
    _1 = 0,
    #[doc = "1: 2 stop bits (1.5 if LCR\\[1:0\\]=00)."]
    _2 = 1,
}
impl From<SBS_A> for bool {
    #[inline(always)]
    fn from(variant: SBS_A) -> Self {
        variant as u8 != 0
    }
}
impl SBS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBS_A {
        match self.bits {
            false => SBS_A::_1,
            true => SBS_A::_2,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SBS_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == SBS_A::_2
    }
}
#[doc = "Field `SBS` writer - Stop Bit Select"]
pub type SBS_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCR_SPEC, SBS_A, O>;
impl<'a, const O: u8> SBS_W<'a, O> {
    #[doc = "1 stop bit."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBS_A::_1)
    }
    #[doc = "2 stop bits (1.5 if LCR\\[1:0\\]=00)."]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(SBS_A::_2)
    }
}
#[doc = "Field `PE` reader - Parity Enable"]
pub type PE_R = crate::BitReader<PE_A>;
#[doc = "Parity Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PE_A {
    #[doc = "0: Disable parity generation and checking."]
    DISABLED = 0,
    #[doc = "1: Enable parity generation and checking."]
    ENABLED = 1,
}
impl From<PE_A> for bool {
    #[inline(always)]
    fn from(variant: PE_A) -> Self {
        variant as u8 != 0
    }
}
impl PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PE_A {
        match self.bits {
            false => PE_A::DISABLED,
            true => PE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PE_A::ENABLED
    }
}
#[doc = "Field `PE` writer - Parity Enable"]
pub type PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCR_SPEC, PE_A, O>;
impl<'a, const O: u8> PE_W<'a, O> {
    #[doc = "Disable parity generation and checking."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PE_A::DISABLED)
    }
    #[doc = "Enable parity generation and checking."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PE_A::ENABLED)
    }
}
#[doc = "Field `PS` reader - Parity Select"]
pub type PS_R = crate::FieldReader<u8, PS_A>;
#[doc = "Parity Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PS_A {
    #[doc = "0: Odd parity. Number of 1s in the transmitted character and the attached parity bit will be odd."]
    ODD = 0,
    #[doc = "1: Even Parity. Number of 1s in the transmitted character and the attached parity bit will be even."]
    EVEN = 1,
    #[doc = "2: Forced 1 stick parity."]
    FORCED_1_STICK = 2,
    #[doc = "3: Forced 0 stick parity."]
    FORCED_0_STICK = 3,
}
impl From<PS_A> for u8 {
    #[inline(always)]
    fn from(variant: PS_A) -> Self {
        variant as _
    }
}
impl PS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS_A {
        match self.bits {
            0 => PS_A::ODD,
            1 => PS_A::EVEN,
            2 => PS_A::FORCED_1_STICK,
            3 => PS_A::FORCED_0_STICK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == PS_A::ODD
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == PS_A::EVEN
    }
    #[doc = "Checks if the value of the field is `FORCED_1_STICK`"]
    #[inline(always)]
    pub fn is_forced_1_stick(&self) -> bool {
        *self == PS_A::FORCED_1_STICK
    }
    #[doc = "Checks if the value of the field is `FORCED_0_STICK`"]
    #[inline(always)]
    pub fn is_forced_0_stick(&self) -> bool {
        *self == PS_A::FORCED_0_STICK
    }
}
#[doc = "Field `PS` writer - Parity Select"]
pub type PS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, LCR_SPEC, u8, PS_A, 2, O>;
impl<'a, const O: u8> PS_W<'a, O> {
    #[doc = "Odd parity. Number of 1s in the transmitted character and the attached parity bit will be odd."]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(PS_A::ODD)
    }
    #[doc = "Even Parity. Number of 1s in the transmitted character and the attached parity bit will be even."]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(PS_A::EVEN)
    }
    #[doc = "Forced 1 stick parity."]
    #[inline(always)]
    pub fn forced_1_stick(self) -> &'a mut W {
        self.variant(PS_A::FORCED_1_STICK)
    }
    #[doc = "Forced 0 stick parity."]
    #[inline(always)]
    pub fn forced_0_stick(self) -> &'a mut W {
        self.variant(PS_A::FORCED_0_STICK)
    }
}
#[doc = "Field `BC` reader - Break Control"]
pub type BC_R = crate::BitReader<BC_A>;
#[doc = "Break Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BC_A {
    #[doc = "0: Disable break transmission."]
    DISABLE = 0,
    #[doc = "1: Enable break transmission. Output pin USART TXD is forced to logic 0 when LCR\\[6\\]
is active high."]
    ENABLE = 1,
}
impl From<BC_A> for bool {
    #[inline(always)]
    fn from(variant: BC_A) -> Self {
        variant as u8 != 0
    }
}
impl BC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BC_A {
        match self.bits {
            false => BC_A::DISABLE,
            true => BC_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BC_A::ENABLE
    }
}
#[doc = "Field `BC` writer - Break Control"]
pub type BC_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCR_SPEC, BC_A, O>;
impl<'a, const O: u8> BC_W<'a, O> {
    #[doc = "Disable break transmission."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BC_A::DISABLE)
    }
    #[doc = "Enable break transmission. Output pin USART TXD is forced to logic 0 when LCR\\[6\\]
is active high."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BC_A::ENABLE)
    }
}
#[doc = "Field `DLAB` reader - Divisor Latch Access Bit"]
pub type DLAB_R = crate::BitReader<DLAB_A>;
#[doc = "Divisor Latch Access Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLAB_A {
    #[doc = "0: Disable access to Divisor Latches."]
    DISABLE = 0,
    #[doc = "1: Enable access to Divisor Latches."]
    ENABLE = 1,
}
impl From<DLAB_A> for bool {
    #[inline(always)]
    fn from(variant: DLAB_A) -> Self {
        variant as u8 != 0
    }
}
impl DLAB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLAB_A {
        match self.bits {
            false => DLAB_A::DISABLE,
            true => DLAB_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DLAB_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DLAB_A::ENABLE
    }
}
#[doc = "Field `DLAB` writer - Divisor Latch Access Bit"]
pub type DLAB_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCR_SPEC, DLAB_A, O>;
impl<'a, const O: u8> DLAB_W<'a, O> {
    #[doc = "Disable access to Divisor Latches."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DLAB_A::DISABLE)
    }
    #[doc = "Enable access to Divisor Latches."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DLAB_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:1 - Word Length Select"]
    #[inline(always)]
    pub fn wls(&self) -> WLS_R {
        WLS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Stop Bit Select"]
    #[inline(always)]
    pub fn sbs(&self) -> SBS_R {
        SBS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Parity Enable"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Parity Select"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Break Control"]
    #[inline(always)]
    pub fn bc(&self) -> BC_R {
        BC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Divisor Latch Access Bit"]
    #[inline(always)]
    pub fn dlab(&self) -> DLAB_R {
        DLAB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Word Length Select"]
    #[inline(always)]
    #[must_use]
    pub fn wls(&mut self) -> WLS_W<0> {
        WLS_W::new(self)
    }
    #[doc = "Bit 2 - Stop Bit Select"]
    #[inline(always)]
    #[must_use]
    pub fn sbs(&mut self) -> SBS_W<2> {
        SBS_W::new(self)
    }
    #[doc = "Bit 3 - Parity Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<3> {
        PE_W::new(self)
    }
    #[doc = "Bits 4:5 - Parity Select"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PS_W<4> {
        PS_W::new(self)
    }
    #[doc = "Bit 6 - Break Control"]
    #[inline(always)]
    #[must_use]
    pub fn bc(&mut self) -> BC_W<6> {
        BC_W::new(self)
    }
    #[doc = "Bit 7 - Divisor Latch Access Bit"]
    #[inline(always)]
    #[must_use]
    pub fn dlab(&mut self) -> DLAB_W<7> {
        DLAB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Line Control Register. Contains controls for frame formatting and break generation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcr](index.html) module"]
pub struct LCR_SPEC;
impl crate::RegisterSpec for LCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcr::R](R) reader structure"]
impl crate::Readable for LCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcr::W](W) writer structure"]
impl crate::Writable for LCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCR to value 0"]
impl crate::Resettable for LCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
