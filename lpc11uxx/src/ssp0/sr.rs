#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TFE` reader - Transmit FIFO Empty. This bit is 1 is the Transmit FIFO is empty, 0 if not."]
pub type TFE_R = crate::BitReader<bool>;
#[doc = "Field `TNF` reader - Transmit FIFO Not Full. This bit is 0 if the Tx FIFO is full, 1 if not."]
pub type TNF_R = crate::BitReader<bool>;
#[doc = "Field `RNE` reader - Receive FIFO Not Empty. This bit is 0 if the Receive FIFO is empty, 1 if not."]
pub type RNE_R = crate::BitReader<bool>;
#[doc = "Field `RFF` reader - Receive FIFO Full. This bit is 1 if the Receive FIFO is full, 0 if not."]
pub type RFF_R = crate::BitReader<bool>;
#[doc = "Field `BSY` reader - Busy. This bit is 0 if the SPI controller is idle, 1 if it is currently sending/receiving a frame and/or the Tx FIFO is not empty."]
pub type BSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Transmit FIFO Empty. This bit is 1 is the Transmit FIFO is empty, 0 if not."]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Not Full. This bit is 0 if the Tx FIFO is full, 1 if not."]
    #[inline(always)]
    pub fn tnf(&self) -> TNF_R {
        TNF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO Not Empty. This bit is 0 if the Receive FIFO is empty, 1 if not."]
    #[inline(always)]
    pub fn rne(&self) -> RNE_R {
        RNE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO Full. This bit is 1 if the Receive FIFO is full, 0 if not."]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Busy. This bit is 0 if the SPI controller is idle, 1 if it is currently sending/receiving a frame and/or the Tx FIFO is not empty."]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0x03"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
