#[doc = "Register `GDR` reader"]
pub struct R(crate::R<GDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GDR` writer"]
pub struct W(crate::W<GDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GDR_SPEC>;
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
impl From<crate::W<GDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `V_VREF` reader - When DONE is 1, this field contains a binary fraction representing the voltage on the ADn pin selected by the SEL field, divided by the voltage on the VDD pin. Zero in the field indicates that the voltage on the ADn pin was less than, equal to, or close to that on VSS, while 0x3FF indicates that the voltage on ADn was close to, equal to, or greater than that on VREF."]
pub type V_VREF_R = crate::FieldReader<u16, u16>;
#[doc = "Field `V_VREF` writer - When DONE is 1, this field contains a binary fraction representing the voltage on the ADn pin selected by the SEL field, divided by the voltage on the VDD pin. Zero in the field indicates that the voltage on the ADn pin was less than, equal to, or close to that on VSS, while 0x3FF indicates that the voltage on ADn was close to, equal to, or greater than that on VREF."]
pub type V_VREF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GDR_SPEC, u16, u16, 10, O>;
#[doc = "Field `CHN` reader - These bits contain the channel from which the result bits V_VREF were converted."]
pub type CHN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHN` writer - These bits contain the channel from which the result bits V_VREF were converted."]
pub type CHN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GDR_SPEC, u8, u8, 3, O>;
#[doc = "Field `OVERRUN` reader - This bit is 1 in burst mode if the results of one or more conversions was (were) lost and overwritten before the conversion that produced the result in the V_VREF bits."]
pub type OVERRUN_R = crate::BitReader<bool>;
#[doc = "Field `OVERRUN` writer - This bit is 1 in burst mode if the results of one or more conversions was (were) lost and overwritten before the conversion that produced the result in the V_VREF bits."]
pub type OVERRUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GDR_SPEC, bool, O>;
#[doc = "Field `DONE` reader - This bit is set to 1 when an A/D conversion completes. It is cleared when this register is read and when the ADCR is written. If the ADCR is written while a conversion is still in progress, this bit is set and a new conversion is started."]
pub type DONE_R = crate::BitReader<bool>;
#[doc = "Field `DONE` writer - This bit is set to 1 when an A/D conversion completes. It is cleared when this register is read and when the ADCR is written. If the ADCR is written while a conversion is still in progress, this bit is set and a new conversion is started."]
pub type DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GDR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 6:15 - When DONE is 1, this field contains a binary fraction representing the voltage on the ADn pin selected by the SEL field, divided by the voltage on the VDD pin. Zero in the field indicates that the voltage on the ADn pin was less than, equal to, or close to that on VSS, while 0x3FF indicates that the voltage on ADn was close to, equal to, or greater than that on VREF."]
    #[inline(always)]
    pub fn v_vref(&self) -> V_VREF_R {
        V_VREF_R::new(((self.bits >> 6) & 0x03ff) as u16)
    }
    #[doc = "Bits 24:26 - These bits contain the channel from which the result bits V_VREF were converted."]
    #[inline(always)]
    pub fn chn(&self) -> CHN_R {
        CHN_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 30 - This bit is 1 in burst mode if the results of one or more conversions was (were) lost and overwritten before the conversion that produced the result in the V_VREF bits."]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This bit is set to 1 when an A/D conversion completes. It is cleared when this register is read and when the ADCR is written. If the ADCR is written while a conversion is still in progress, this bit is set and a new conversion is started."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 6:15 - When DONE is 1, this field contains a binary fraction representing the voltage on the ADn pin selected by the SEL field, divided by the voltage on the VDD pin. Zero in the field indicates that the voltage on the ADn pin was less than, equal to, or close to that on VSS, while 0x3FF indicates that the voltage on ADn was close to, equal to, or greater than that on VREF."]
    #[inline(always)]
    #[must_use]
    pub fn v_vref(&mut self) -> V_VREF_W<6> {
        V_VREF_W::new(self)
    }
    #[doc = "Bits 24:26 - These bits contain the channel from which the result bits V_VREF were converted."]
    #[inline(always)]
    #[must_use]
    pub fn chn(&mut self) -> CHN_W<24> {
        CHN_W::new(self)
    }
    #[doc = "Bit 30 - This bit is 1 in burst mode if the results of one or more conversions was (were) lost and overwritten before the conversion that produced the result in the V_VREF bits."]
    #[inline(always)]
    #[must_use]
    pub fn overrun(&mut self) -> OVERRUN_W<30> {
        OVERRUN_W::new(self)
    }
    #[doc = "Bit 31 - This bit is set to 1 when an A/D conversion completes. It is cleared when this register is read and when the ADCR is written. If the ADCR is written while a conversion is still in progress, this bit is set and a new conversion is started."]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<31> {
        DONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Global Data Register. Contains the result of the most recent A/D conversion.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gdr](index.html) module"]
pub struct GDR_SPEC;
impl crate::RegisterSpec for GDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gdr::R](R) reader structure"]
impl crate::Readable for GDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gdr::W](W) writer structure"]
impl crate::Writable for GDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GDR to value 0"]
impl crate::Resettable for GDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
