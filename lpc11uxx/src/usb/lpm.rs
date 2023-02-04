#[doc = "Register `LPM` reader"]
pub struct R(crate::R<LPM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPM` writer"]
pub struct W(crate::W<LPM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPM_SPEC>;
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
impl From<crate::W<LPM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIRD_HW` reader - Host Initiated Resume Duration - HW. This is the HIRD value from the last received LPM token"]
pub type HIRD_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HIRD_HW` writer - Host Initiated Resume Duration - HW. This is the HIRD value from the last received LPM token"]
pub type HIRD_HW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPM_SPEC, u8, u8, 4, O>;
#[doc = "Field `HIRD_SW` reader - Host Initiated Resume Duration - SW. This is the time duration required by the USB device system to come out of LPM initiated suspend after receiving the host initiated LPM resume."]
pub type HIRD_SW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HIRD_SW` writer - Host Initiated Resume Duration - SW. This is the time duration required by the USB device system to come out of LPM initiated suspend after receiving the host initiated LPM resume."]
pub type HIRD_SW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPM_SPEC, u8, u8, 4, O>;
#[doc = "Field `DATA_PENDING` reader - As long as this bit is set to one and LPM supported bit is set to one, HW will return a NYET handshake on every LPM token it receives. If LPM supported bit is set to one and this bit is zero, HW will return an ACK handshake on every LPM token it receives. If SW has still data pending and LPM is supported, it must set this bit to 1."]
pub type DATA_PENDING_R = crate::BitReader<bool>;
#[doc = "Field `DATA_PENDING` writer - As long as this bit is set to one and LPM supported bit is set to one, HW will return a NYET handshake on every LPM token it receives. If LPM supported bit is set to one and this bit is zero, HW will return an ACK handshake on every LPM token it receives. If SW has still data pending and LPM is supported, it must set this bit to 1."]
pub type DATA_PENDING_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPM_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Host Initiated Resume Duration - HW. This is the HIRD value from the last received LPM token"]
    #[inline(always)]
    pub fn hird_hw(&self) -> HIRD_HW_R {
        HIRD_HW_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Host Initiated Resume Duration - SW. This is the time duration required by the USB device system to come out of LPM initiated suspend after receiving the host initiated LPM resume."]
    #[inline(always)]
    pub fn hird_sw(&self) -> HIRD_SW_R {
        HIRD_SW_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - As long as this bit is set to one and LPM supported bit is set to one, HW will return a NYET handshake on every LPM token it receives. If LPM supported bit is set to one and this bit is zero, HW will return an ACK handshake on every LPM token it receives. If SW has still data pending and LPM is supported, it must set this bit to 1."]
    #[inline(always)]
    pub fn data_pending(&self) -> DATA_PENDING_R {
        DATA_PENDING_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Host Initiated Resume Duration - HW. This is the HIRD value from the last received LPM token"]
    #[inline(always)]
    #[must_use]
    pub fn hird_hw(&mut self) -> HIRD_HW_W<0> {
        HIRD_HW_W::new(self)
    }
    #[doc = "Bits 4:7 - Host Initiated Resume Duration - SW. This is the time duration required by the USB device system to come out of LPM initiated suspend after receiving the host initiated LPM resume."]
    #[inline(always)]
    #[must_use]
    pub fn hird_sw(&mut self) -> HIRD_SW_W<4> {
        HIRD_SW_W::new(self)
    }
    #[doc = "Bit 8 - As long as this bit is set to one and LPM supported bit is set to one, HW will return a NYET handshake on every LPM token it receives. If LPM supported bit is set to one and this bit is zero, HW will return an ACK handshake on every LPM token it receives. If SW has still data pending and LPM is supported, it must set this bit to 1."]
    #[inline(always)]
    #[must_use]
    pub fn data_pending(&mut self) -> DATA_PENDING_W<8> {
        DATA_PENDING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Link Power Management register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpm](index.html) module"]
pub struct LPM_SPEC;
impl crate::RegisterSpec for LPM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpm::R](R) reader structure"]
impl crate::Readable for LPM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpm::W](W) writer structure"]
impl crate::Writable for LPM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPM to value 0"]
impl crate::Resettable for LPM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
