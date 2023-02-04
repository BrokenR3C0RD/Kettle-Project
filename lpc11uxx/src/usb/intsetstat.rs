#[doc = "Register `INTSETSTAT` reader"]
pub struct R(crate::R<INTSETSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSETSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTSETSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTSETSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTSETSTAT` writer"]
pub struct W(crate::W<INTSETSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTSETSTAT_SPEC>;
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
impl From<crate::W<INTSETSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTSETSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EP_SET_INT` reader - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set. When this register is read, the same value as the USB interrupt status register is returned."]
pub type EP_SET_INT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EP_SET_INT` writer - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set. When this register is read, the same value as the USB interrupt status register is returned."]
pub type EP_SET_INT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTSETSTAT_SPEC, u16, u16, 10, O>;
#[doc = "Field `FRAME_SET_INT` reader - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set. When this register is read, the same value as the USB interrupt status register is returned."]
pub type FRAME_SET_INT_R = crate::BitReader<bool>;
#[doc = "Field `FRAME_SET_INT` writer - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set. When this register is read, the same value as the USB interrupt status register is returned."]
pub type FRAME_SET_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSETSTAT_SPEC, bool, O>;
#[doc = "Field `DEV_SET_INT` reader - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set. When this register is read, the same value as the USB interrupt status register is returned."]
pub type DEV_SET_INT_R = crate::BitReader<bool>;
#[doc = "Field `DEV_SET_INT` writer - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set. When this register is read, the same value as the USB interrupt status register is returned."]
pub type DEV_SET_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSETSTAT_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9 - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set. When this register is read, the same value as the USB interrupt status register is returned."]
    #[inline(always)]
    pub fn ep_set_int(&self) -> EP_SET_INT_R {
        EP_SET_INT_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 30 - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set. When this register is read, the same value as the USB interrupt status register is returned."]
    #[inline(always)]
    pub fn frame_set_int(&self) -> FRAME_SET_INT_R {
        FRAME_SET_INT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set. When this register is read, the same value as the USB interrupt status register is returned."]
    #[inline(always)]
    pub fn dev_set_int(&self) -> DEV_SET_INT_R {
        DEV_SET_INT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set. When this register is read, the same value as the USB interrupt status register is returned."]
    #[inline(always)]
    #[must_use]
    pub fn ep_set_int(&mut self) -> EP_SET_INT_W<0> {
        EP_SET_INT_W::new(self)
    }
    #[doc = "Bit 30 - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set. When this register is read, the same value as the USB interrupt status register is returned."]
    #[inline(always)]
    #[must_use]
    pub fn frame_set_int(&mut self) -> FRAME_SET_INT_W<30> {
        FRAME_SET_INT_W::new(self)
    }
    #[doc = "Bit 31 - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set. When this register is read, the same value as the USB interrupt status register is returned."]
    #[inline(always)]
    #[must_use]
    pub fn dev_set_int(&mut self) -> DEV_SET_INT_W<31> {
        DEV_SET_INT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB set interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intsetstat](index.html) module"]
pub struct INTSETSTAT_SPEC;
impl crate::RegisterSpec for INTSETSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intsetstat::R](R) reader structure"]
impl crate::Readable for INTSETSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intsetstat::W](W) writer structure"]
impl crate::Writable for INTSETSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTSETSTAT to value 0"]
impl crate::Resettable for INTSETSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
