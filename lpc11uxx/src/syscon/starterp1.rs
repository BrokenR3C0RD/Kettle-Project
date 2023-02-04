#[doc = "Register `STARTERP1` reader"]
pub struct R(crate::R<STARTERP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STARTERP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STARTERP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STARTERP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STARTERP1` writer"]
pub struct W(crate::W<STARTERP1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STARTERP1_SPEC>;
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
impl From<crate::W<STARTERP1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STARTERP1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WWDTINT` reader - WWDT interrupt wake-up"]
pub type WWDTINT_R = crate::BitReader<WWDTINT_A>;
#[doc = "WWDT interrupt wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDTINT_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<WWDTINT_A> for bool {
    #[inline(always)]
    fn from(variant: WWDTINT_A) -> Self {
        variant as u8 != 0
    }
}
impl WWDTINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WWDTINT_A {
        match self.bits {
            false => WWDTINT_A::DISABLED,
            true => WWDTINT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WWDTINT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WWDTINT_A::ENABLED
    }
}
#[doc = "Field `WWDTINT` writer - WWDT interrupt wake-up"]
pub type WWDTINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP1_SPEC, WWDTINT_A, O>;
impl<'a, const O: u8> WWDTINT_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WWDTINT_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WWDTINT_A::ENABLED)
    }
}
#[doc = "Field `BODINT` reader - Brown Out Detect (BOD) interrupt wake-up"]
pub type BODINT_R = crate::BitReader<BODINT_A>;
#[doc = "Brown Out Detect (BOD) interrupt wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BODINT_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<BODINT_A> for bool {
    #[inline(always)]
    fn from(variant: BODINT_A) -> Self {
        variant as u8 != 0
    }
}
impl BODINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODINT_A {
        match self.bits {
            false => BODINT_A::DISABLED,
            true => BODINT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BODINT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BODINT_A::ENABLED
    }
}
#[doc = "Field `BODINT` writer - Brown Out Detect (BOD) interrupt wake-up"]
pub type BODINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP1_SPEC, BODINT_A, O>;
impl<'a, const O: u8> BODINT_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BODINT_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BODINT_A::ENABLED)
    }
}
#[doc = "Field `USB_WAKEUP` reader - USB need_clock signal wake-up"]
pub type USB_WAKEUP_R = crate::BitReader<USB_WAKEUP_A>;
#[doc = "USB need_clock signal wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USB_WAKEUP_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<USB_WAKEUP_A> for bool {
    #[inline(always)]
    fn from(variant: USB_WAKEUP_A) -> Self {
        variant as u8 != 0
    }
}
impl USB_WAKEUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB_WAKEUP_A {
        match self.bits {
            false => USB_WAKEUP_A::DISABLED,
            true => USB_WAKEUP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USB_WAKEUP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USB_WAKEUP_A::ENABLED
    }
}
#[doc = "Field `USB_WAKEUP` writer - USB need_clock signal wake-up"]
pub type USB_WAKEUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP1_SPEC, USB_WAKEUP_A, O>;
impl<'a, const O: u8> USB_WAKEUP_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USB_WAKEUP_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USB_WAKEUP_A::ENABLED)
    }
}
#[doc = "Field `GPIOINT0` reader - GPIO GROUP0 interrupt wake-up"]
pub type GPIOINT0_R = crate::BitReader<GPIOINT0_A>;
#[doc = "GPIO GROUP0 interrupt wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOINT0_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<GPIOINT0_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOINT0_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIOINT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOINT0_A {
        match self.bits {
            false => GPIOINT0_A::DISABLED,
            true => GPIOINT0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOINT0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOINT0_A::ENABLED
    }
}
#[doc = "Field `GPIOINT0` writer - GPIO GROUP0 interrupt wake-up"]
pub type GPIOINT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP1_SPEC, GPIOINT0_A, O>;
impl<'a, const O: u8> GPIOINT0_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOINT0_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOINT0_A::ENABLED)
    }
}
#[doc = "Field `GPIOINT1` reader - GPIO GROUP1 interrupt wake-up"]
pub type GPIOINT1_R = crate::BitReader<GPIOINT1_A>;
#[doc = "GPIO GROUP1 interrupt wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOINT1_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<GPIOINT1_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOINT1_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIOINT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOINT1_A {
        match self.bits {
            false => GPIOINT1_A::DISABLED,
            true => GPIOINT1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOINT1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOINT1_A::ENABLED
    }
}
#[doc = "Field `GPIOINT1` writer - GPIO GROUP1 interrupt wake-up"]
pub type GPIOINT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP1_SPEC, GPIOINT1_A, O>;
impl<'a, const O: u8> GPIOINT1_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOINT1_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOINT1_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 12 - WWDT interrupt wake-up"]
    #[inline(always)]
    pub fn wwdtint(&self) -> WWDTINT_R {
        WWDTINT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Brown Out Detect (BOD) interrupt wake-up"]
    #[inline(always)]
    pub fn bodint(&self) -> BODINT_R {
        BODINT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 19 - USB need_clock signal wake-up"]
    #[inline(always)]
    pub fn usb_wakeup(&self) -> USB_WAKEUP_R {
        USB_WAKEUP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPIO GROUP0 interrupt wake-up"]
    #[inline(always)]
    pub fn gpioint0(&self) -> GPIOINT0_R {
        GPIOINT0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - GPIO GROUP1 interrupt wake-up"]
    #[inline(always)]
    pub fn gpioint1(&self) -> GPIOINT1_R {
        GPIOINT1_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - WWDT interrupt wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn wwdtint(&mut self) -> WWDTINT_W<12> {
        WWDTINT_W::new(self)
    }
    #[doc = "Bit 13 - Brown Out Detect (BOD) interrupt wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn bodint(&mut self) -> BODINT_W<13> {
        BODINT_W::new(self)
    }
    #[doc = "Bit 19 - USB need_clock signal wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn usb_wakeup(&mut self) -> USB_WAKEUP_W<19> {
        USB_WAKEUP_W::new(self)
    }
    #[doc = "Bit 20 - GPIO GROUP0 interrupt wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn gpioint0(&mut self) -> GPIOINT0_W<20> {
        GPIOINT0_W::new(self)
    }
    #[doc = "Bit 21 - GPIO GROUP1 interrupt wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn gpioint1(&mut self) -> GPIOINT1_W<21> {
        GPIOINT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start logic 1 interrupt wake-up enable register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [starterp1](index.html) module"]
pub struct STARTERP1_SPEC;
impl crate::RegisterSpec for STARTERP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [starterp1::R](R) reader structure"]
impl crate::Readable for STARTERP1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [starterp1::W](W) writer structure"]
impl crate::Writable for STARTERP1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STARTERP1 to value 0"]
impl crate::Resettable for STARTERP1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
