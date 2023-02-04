#[doc = "Register `IR` reader"]
pub struct R(crate::R<IR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IR` writer"]
pub struct W(crate::W<IR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IR_SPEC>;
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
impl From<crate::W<IR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MR0INT` reader - Interrupt flag for match channel 0."]
pub type MR0INT_R = crate::BitReader<bool>;
#[doc = "Field `MR0INT` writer - Interrupt flag for match channel 0."]
pub type MR0INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `MR1INT` reader - Interrupt flag for match channel 1."]
pub type MR1INT_R = crate::BitReader<bool>;
#[doc = "Field `MR1INT` writer - Interrupt flag for match channel 1."]
pub type MR1INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `MR2INT` reader - Interrupt flag for match channel 2."]
pub type MR2INT_R = crate::BitReader<bool>;
#[doc = "Field `MR2INT` writer - Interrupt flag for match channel 2."]
pub type MR2INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `MR3INT` reader - Interrupt flag for match channel 3."]
pub type MR3INT_R = crate::BitReader<bool>;
#[doc = "Field `MR3INT` writer - Interrupt flag for match channel 3."]
pub type MR3INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `CR0INT` reader - Interrupt flag for capture channel 0 event."]
pub type CR0INT_R = crate::BitReader<bool>;
#[doc = "Field `CR0INT` writer - Interrupt flag for capture channel 0 event."]
pub type CR0INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `CR1INT` reader - Interrupt flag for capture channel 1 event."]
pub type CR1INT_R = crate::BitReader<bool>;
#[doc = "Field `CR1INT` writer - Interrupt flag for capture channel 1 event."]
pub type CR1INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Interrupt flag for match channel 0."]
    #[inline(always)]
    pub fn mr0int(&self) -> MR0INT_R {
        MR0INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt flag for match channel 1."]
    #[inline(always)]
    pub fn mr1int(&self) -> MR1INT_R {
        MR1INT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt flag for match channel 2."]
    #[inline(always)]
    pub fn mr2int(&self) -> MR2INT_R {
        MR2INT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt flag for match channel 3."]
    #[inline(always)]
    pub fn mr3int(&self) -> MR3INT_R {
        MR3INT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt flag for capture channel 0 event."]
    #[inline(always)]
    pub fn cr0int(&self) -> CR0INT_R {
        CR0INT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt flag for capture channel 1 event."]
    #[inline(always)]
    pub fn cr1int(&self) -> CR1INT_R {
        CR1INT_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt flag for match channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn mr0int(&mut self) -> MR0INT_W<0> {
        MR0INT_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt flag for match channel 1."]
    #[inline(always)]
    #[must_use]
    pub fn mr1int(&mut self) -> MR1INT_W<1> {
        MR1INT_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt flag for match channel 2."]
    #[inline(always)]
    #[must_use]
    pub fn mr2int(&mut self) -> MR2INT_W<2> {
        MR2INT_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt flag for match channel 3."]
    #[inline(always)]
    #[must_use]
    pub fn mr3int(&mut self) -> MR3INT_W<3> {
        MR3INT_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt flag for capture channel 0 event."]
    #[inline(always)]
    #[must_use]
    pub fn cr0int(&mut self) -> CR0INT_W<4> {
        CR0INT_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt flag for capture channel 1 event."]
    #[inline(always)]
    #[must_use]
    pub fn cr1int(&mut self) -> CR1INT_W<6> {
        CR1INT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Register. The IR can be written to clear interrupts. The IR can be read to identify which of eight possible interrupt sources are pending.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ir](index.html) module"]
pub struct IR_SPEC;
impl crate::RegisterSpec for IR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ir::R](R) reader structure"]
impl crate::Readable for IR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ir::W](W) writer structure"]
impl crate::Writable for IR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IR to value 0"]
impl crate::Resettable for IR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
