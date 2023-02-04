#[doc = "Register `RS485DLY` reader"]
pub struct R(crate::R<RS485DLY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RS485DLY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RS485DLY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RS485DLY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RS485DLY` writer"]
pub struct W(crate::W<RS485DLY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RS485DLY_SPEC>;
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
impl From<crate::W<RS485DLY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RS485DLY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLY` reader - Contains the direction control (RTS or DTR) delay value. This register works in conjunction with an 8-bit counter."]
pub type DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLY` writer - Contains the direction control (RTS or DTR) delay value. This register works in conjunction with an 8-bit counter."]
pub type DLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RS485DLY_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Contains the direction control (RTS or DTR) delay value. This register works in conjunction with an 8-bit counter."]
    #[inline(always)]
    pub fn dly(&self) -> DLY_R {
        DLY_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Contains the direction control (RTS or DTR) delay value. This register works in conjunction with an 8-bit counter."]
    #[inline(always)]
    #[must_use]
    pub fn dly(&mut self) -> DLY_W<0> {
        DLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RS-485/EIA-485 direction control delay.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rs485dly](index.html) module"]
pub struct RS485DLY_SPEC;
impl crate::RegisterSpec for RS485DLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rs485dly::R](R) reader structure"]
impl crate::Readable for RS485DLY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rs485dly::W](W) writer structure"]
impl crate::Writable for RS485DLY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RS485DLY to value 0"]
impl crate::Resettable for RS485DLY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
