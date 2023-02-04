#[doc = "Register `INTROUTING` reader"]
pub struct R(crate::R<INTROUTING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTROUTING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTROUTING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTROUTING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTROUTING` writer"]
pub struct W(crate::W<INTROUTING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTROUTING_SPEC>;
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
impl From<crate::W<INTROUTING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTROUTING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROUTE_INT9_0` reader - This bit can control on which hardware interrupt line the interrupt will be generated: 0: IRQ interrupt line is selected for this interrupt bit 1: FIQ interrupt line is selected for this interrupt bit"]
pub type ROUTE_INT9_0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ROUTE_INT9_0` writer - This bit can control on which hardware interrupt line the interrupt will be generated: 0: IRQ interrupt line is selected for this interrupt bit 1: FIQ interrupt line is selected for this interrupt bit"]
pub type ROUTE_INT9_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTROUTING_SPEC, u16, u16, 10, O>;
#[doc = "Field `ROUTE_INT30` reader - This bit can control on which hardware interrupt line the interrupt will be generated: 0: IRQ interrupt line is selected for this interrupt bit 1: FIQ interrupt line is selected for this interrupt bit"]
pub type ROUTE_INT30_R = crate::BitReader<bool>;
#[doc = "Field `ROUTE_INT30` writer - This bit can control on which hardware interrupt line the interrupt will be generated: 0: IRQ interrupt line is selected for this interrupt bit 1: FIQ interrupt line is selected for this interrupt bit"]
pub type ROUTE_INT30_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTROUTING_SPEC, bool, O>;
#[doc = "Field `ROUTE_INT31` reader - This bit can control on which hardware interrupt line the interrupt will be generated: 0: IRQ interrupt line is selected for this interrupt bit 1: FIQ interrupt line is selected for this interrupt bit"]
pub type ROUTE_INT31_R = crate::BitReader<bool>;
#[doc = "Field `ROUTE_INT31` writer - This bit can control on which hardware interrupt line the interrupt will be generated: 0: IRQ interrupt line is selected for this interrupt bit 1: FIQ interrupt line is selected for this interrupt bit"]
pub type ROUTE_INT31_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTROUTING_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9 - This bit can control on which hardware interrupt line the interrupt will be generated: 0: IRQ interrupt line is selected for this interrupt bit 1: FIQ interrupt line is selected for this interrupt bit"]
    #[inline(always)]
    pub fn route_int9_0(&self) -> ROUTE_INT9_0_R {
        ROUTE_INT9_0_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 30 - This bit can control on which hardware interrupt line the interrupt will be generated: 0: IRQ interrupt line is selected for this interrupt bit 1: FIQ interrupt line is selected for this interrupt bit"]
    #[inline(always)]
    pub fn route_int30(&self) -> ROUTE_INT30_R {
        ROUTE_INT30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This bit can control on which hardware interrupt line the interrupt will be generated: 0: IRQ interrupt line is selected for this interrupt bit 1: FIQ interrupt line is selected for this interrupt bit"]
    #[inline(always)]
    pub fn route_int31(&self) -> ROUTE_INT31_R {
        ROUTE_INT31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - This bit can control on which hardware interrupt line the interrupt will be generated: 0: IRQ interrupt line is selected for this interrupt bit 1: FIQ interrupt line is selected for this interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn route_int9_0(&mut self) -> ROUTE_INT9_0_W<0> {
        ROUTE_INT9_0_W::new(self)
    }
    #[doc = "Bit 30 - This bit can control on which hardware interrupt line the interrupt will be generated: 0: IRQ interrupt line is selected for this interrupt bit 1: FIQ interrupt line is selected for this interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn route_int30(&mut self) -> ROUTE_INT30_W<30> {
        ROUTE_INT30_W::new(self)
    }
    #[doc = "Bit 31 - This bit can control on which hardware interrupt line the interrupt will be generated: 0: IRQ interrupt line is selected for this interrupt bit 1: FIQ interrupt line is selected for this interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn route_int31(&mut self) -> ROUTE_INT31_W<31> {
        ROUTE_INT31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB interrupt routing register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [introuting](index.html) module"]
pub struct INTROUTING_SPEC;
impl crate::RegisterSpec for INTROUTING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [introuting::R](R) reader structure"]
impl crate::Readable for INTROUTING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [introuting::W](W) writer structure"]
impl crate::Writable for INTROUTING_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTROUTING to value 0"]
impl crate::Resettable for INTROUTING_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
