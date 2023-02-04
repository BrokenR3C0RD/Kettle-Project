#[doc = "Register `FCR` writer"]
pub struct W(crate::W<FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCR_SPEC>;
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
impl From<crate::W<FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "FIFO enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOEN_AW {
    #[doc = "0: USART FIFOs are disabled. Must not be used in the application."]
    DISABLED = 0,
    #[doc = "1: Active high enable for both USART Rx and TX FIFOs and FCR\\[7:1\\]
access. This bit must be set for proper USART operation. Any transition on this bit will automatically clear the USART FIFOs."]
    ENABLED = 1,
}
impl From<FIFOEN_AW> for bool {
    #[inline(always)]
    fn from(variant: FIFOEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOEN` writer - FIFO enable"]
pub type FIFOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, FIFOEN_AW, O>;
impl<'a, const O: u8> FIFOEN_W<'a, O> {
    #[doc = "USART FIFOs are disabled. Must not be used in the application."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FIFOEN_AW::DISABLED)
    }
    #[doc = "Active high enable for both USART Rx and TX FIFOs and FCR\\[7:1\\]
access. This bit must be set for proper USART operation. Any transition on this bit will automatically clear the USART FIFOs."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FIFOEN_AW::ENABLED)
    }
}
#[doc = "RX FIFO Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFIFORES_AW {
    #[doc = "0: No impact on either of USART FIFOs."]
    NO_IMPACT = 0,
    #[doc = "1: Writing a logic 1 to FCR\\[1\\]
will clear all bytes in USART Rx FIFO, reset the pointer logic. This bit is self-clearing."]
    CLEAR = 1,
}
impl From<RXFIFORES_AW> for bool {
    #[inline(always)]
    fn from(variant: RXFIFORES_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFIFORES` writer - RX FIFO Reset"]
pub type RXFIFORES_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, RXFIFORES_AW, O>;
impl<'a, const O: u8> RXFIFORES_W<'a, O> {
    #[doc = "No impact on either of USART FIFOs."]
    #[inline(always)]
    pub fn no_impact(self) -> &'a mut W {
        self.variant(RXFIFORES_AW::NO_IMPACT)
    }
    #[doc = "Writing a logic 1 to FCR\\[1\\]
will clear all bytes in USART Rx FIFO, reset the pointer logic. This bit is self-clearing."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RXFIFORES_AW::CLEAR)
    }
}
#[doc = "TX FIFO Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFIFORES_AW {
    #[doc = "0: No impact on either of USART FIFOs."]
    NO_IMPACT = 0,
    #[doc = "1: Writing a logic 1 to FCR\\[2\\]
will clear all bytes in USART TX FIFO, reset the pointer logic. This bit is self-clearing."]
    CLEAR = 1,
}
impl From<TXFIFORES_AW> for bool {
    #[inline(always)]
    fn from(variant: TXFIFORES_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFIFORES` writer - TX FIFO Reset"]
pub type TXFIFORES_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, TXFIFORES_AW, O>;
impl<'a, const O: u8> TXFIFORES_W<'a, O> {
    #[doc = "No impact on either of USART FIFOs."]
    #[inline(always)]
    pub fn no_impact(self) -> &'a mut W {
        self.variant(TXFIFORES_AW::NO_IMPACT)
    }
    #[doc = "Writing a logic 1 to FCR\\[2\\]
will clear all bytes in USART TX FIFO, reset the pointer logic. This bit is self-clearing."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TXFIFORES_AW::CLEAR)
    }
}
#[doc = "RX Trigger Level. These two bits determine how many receiver USART FIFO characters must be written before an interrupt is activated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXTL_AW {
    #[doc = "0: Trigger level 0 (1 character or 0x01)."]
    LEVEL0 = 0,
    #[doc = "1: Trigger level 1 (4 characters or 0x04)."]
    LEVEL1 = 1,
    #[doc = "2: Trigger level 2 (8 characters or 0x08)."]
    LEVEL2 = 2,
    #[doc = "3: Trigger level 3 (14 characters or 0x0E)."]
    LEVEL3 = 3,
}
impl From<RXTL_AW> for u8 {
    #[inline(always)]
    fn from(variant: RXTL_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `RXTL` writer - RX Trigger Level. These two bits determine how many receiver USART FIFO characters must be written before an interrupt is activated."]
pub type RXTL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, FCR_SPEC, u8, RXTL_AW, 2, O>;
impl<'a, const O: u8> RXTL_W<'a, O> {
    #[doc = "Trigger level 0 (1 character or 0x01)."]
    #[inline(always)]
    pub fn level0(self) -> &'a mut W {
        self.variant(RXTL_AW::LEVEL0)
    }
    #[doc = "Trigger level 1 (4 characters or 0x04)."]
    #[inline(always)]
    pub fn level1(self) -> &'a mut W {
        self.variant(RXTL_AW::LEVEL1)
    }
    #[doc = "Trigger level 2 (8 characters or 0x08)."]
    #[inline(always)]
    pub fn level2(self) -> &'a mut W {
        self.variant(RXTL_AW::LEVEL2)
    }
    #[doc = "Trigger level 3 (14 characters or 0x0E)."]
    #[inline(always)]
    pub fn level3(self) -> &'a mut W {
        self.variant(RXTL_AW::LEVEL3)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO enable"]
    #[inline(always)]
    #[must_use]
    pub fn fifoen(&mut self) -> FIFOEN_W<0> {
        FIFOEN_W::new(self)
    }
    #[doc = "Bit 1 - RX FIFO Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifores(&mut self) -> RXFIFORES_W<1> {
        RXFIFORES_W::new(self)
    }
    #[doc = "Bit 2 - TX FIFO Reset"]
    #[inline(always)]
    #[must_use]
    pub fn txfifores(&mut self) -> TXFIFORES_W<2> {
        TXFIFORES_W::new(self)
    }
    #[doc = "Bits 6:7 - RX Trigger Level. These two bits determine how many receiver USART FIFO characters must be written before an interrupt is activated."]
    #[inline(always)]
    #[must_use]
    pub fn rxtl(&mut self) -> RXTL_W<6> {
        RXTL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Control Register. Controls USART FIFO usage and modes.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr](index.html) module"]
pub struct FCR_SPEC;
impl crate::RegisterSpec for FCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [fcr::W](W) writer structure"]
impl crate::Writable for FCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCR to value 0"]
impl crate::Resettable for FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
