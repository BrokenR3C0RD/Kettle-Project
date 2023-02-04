#[doc = "Register `NMISRC` reader"]
pub struct R(crate::R<NMISRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NMISRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NMISRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NMISRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NMISRC` writer"]
pub struct W(crate::W<NMISRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NMISRC_SPEC>;
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
impl From<crate::W<NMISRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NMISRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRQNO` reader - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) if bit 31 is 1. See Table 58 for the list of interrupt sources and their IRQ numbers."]
pub type IRQNO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IRQNO` writer - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) if bit 31 is 1. See Table 58 for the list of interrupt sources and their IRQ numbers."]
pub type IRQNO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NMISRC_SPEC, u8, u8, 5, O>;
#[doc = "Field `NMIEN` reader - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by bits 4:0."]
pub type NMIEN_R = crate::BitReader<bool>;
#[doc = "Field `NMIEN` writer - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by bits 4:0."]
pub type NMIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, NMISRC_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) if bit 31 is 1. See Table 58 for the list of interrupt sources and their IRQ numbers."]
    #[inline(always)]
    pub fn irqno(&self) -> IRQNO_R {
        IRQNO_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by bits 4:0."]
    #[inline(always)]
    pub fn nmien(&self) -> NMIEN_R {
        NMIEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) if bit 31 is 1. See Table 58 for the list of interrupt sources and their IRQ numbers."]
    #[inline(always)]
    #[must_use]
    pub fn irqno(&mut self) -> IRQNO_W<0> {
        IRQNO_W::new(self)
    }
    #[doc = "Bit 31 - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by bits 4:0."]
    #[inline(always)]
    #[must_use]
    pub fn nmien(&mut self) -> NMIEN_W<31> {
        NMIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NMI Source Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nmisrc](index.html) module"]
pub struct NMISRC_SPEC;
impl crate::RegisterSpec for NMISRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nmisrc::R](R) reader structure"]
impl crate::Readable for NMISRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nmisrc::W](W) writer structure"]
impl crate::Writable for NMISRC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NMISRC to value 0"]
impl crate::Resettable for NMISRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
