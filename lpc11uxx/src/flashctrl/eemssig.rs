#[doc = "Register `EEMSSIG` reader"]
pub struct R(crate::R<EEMSSIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEMSSIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEMSSIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEMSSIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA_SIG` reader - BIST 16-bit signature calculated from only the data bytes"]
pub type DATA_SIG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PARITY_SIG` reader - BIST 16-bit signature calculated from only the parity bits of the data bytes"]
pub type PARITY_SIG_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - BIST 16-bit signature calculated from only the data bytes"]
    #[inline(always)]
    pub fn data_sig(&self) -> DATA_SIG_R {
        DATA_SIG_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - BIST 16-bit signature calculated from only the parity bits of the data bytes"]
    #[inline(always)]
    pub fn parity_sig(&self) -> PARITY_SIG_R {
        PARITY_SIG_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "EEPROM 24-bit BIST signature register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eemssig](index.html) module"]
pub struct EEMSSIG_SPEC;
impl crate::RegisterSpec for EEMSSIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eemssig::R](R) reader structure"]
impl crate::Readable for EEMSSIG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EEMSSIG to value 0"]
impl crate::Resettable for EEMSSIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
