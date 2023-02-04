#[doc = "Register `IST` reader"]
pub struct R(crate::R<IST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IST` writer"]
pub struct W(crate::W<IST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IST_SPEC>;
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
impl From<crate::W<IST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSTAT0` reader - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the PINTENT_F register)."]
pub type PSTAT0_R = crate::BitReader<bool>;
#[doc = "Field `PSTAT0` writer - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the PINTENT_F register)."]
pub type PSTAT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IST_SPEC, bool, O>;
#[doc = "Field `PSTAT1` reader - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the PINTENT_F register)."]
pub type PSTAT1_R = crate::BitReader<bool>;
#[doc = "Field `PSTAT1` writer - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the PINTENT_F register)."]
pub type PSTAT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IST_SPEC, bool, O>;
#[doc = "Field `PSTAT2` reader - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the PINTENT_F register)."]
pub type PSTAT2_R = crate::BitReader<bool>;
#[doc = "Field `PSTAT2` writer - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the PINTENT_F register)."]
pub type PSTAT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IST_SPEC, bool, O>;
#[doc = "Field `PSTAT3` reader - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the PINTENT_F register)."]
pub type PSTAT3_R = crate::BitReader<bool>;
#[doc = "Field `PSTAT3` writer - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the PINTENT_F register)."]
pub type PSTAT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IST_SPEC, bool, O>;
#[doc = "Field `PSTAT4` reader - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the PINTENT_F register)."]
pub type PSTAT4_R = crate::BitReader<bool>;
#[doc = "Field `PSTAT4` writer - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the PINTENT_F register)."]
pub type PSTAT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IST_SPEC, bool, O>;
#[doc = "Field `PSTAT5` reader - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the PINTENT_F register)."]
pub type PSTAT5_R = crate::BitReader<bool>;
#[doc = "Field `PSTAT5` writer - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the PINTENT_F register)."]
pub type PSTAT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IST_SPEC, bool, O>;
#[doc = "Field `PSTAT6` reader - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the PINTENT_F register)."]
pub type PSTAT6_R = crate::BitReader<bool>;
#[doc = "Field `PSTAT6` writer - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the PINTENT_F register)."]
pub type PSTAT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IST_SPEC, bool, O>;
#[doc = "Field `PSTAT7` reader - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the PINTENT_F register)."]
pub type PSTAT7_R = crate::BitReader<bool>;
#[doc = "Field `PSTAT7` writer - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the PINTENT_F register)."]
pub type PSTAT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, IST_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the PINTENT_F register)."]
    #[inline(always)]
    pub fn pstat0(&self) -> PSTAT0_R {
        PSTAT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the PINTENT_F register)."]
    #[inline(always)]
    pub fn pstat1(&self) -> PSTAT1_R {
        PSTAT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the PINTENT_F register)."]
    #[inline(always)]
    pub fn pstat2(&self) -> PSTAT2_R {
        PSTAT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the PINTENT_F register)."]
    #[inline(always)]
    pub fn pstat3(&self) -> PSTAT3_R {
        PSTAT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the PINTENT_F register)."]
    #[inline(always)]
    pub fn pstat4(&self) -> PSTAT4_R {
        PSTAT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the PINTENT_F register)."]
    #[inline(always)]
    pub fn pstat5(&self) -> PSTAT5_R {
        PSTAT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the PINTENT_F register)."]
    #[inline(always)]
    pub fn pstat6(&self) -> PSTAT6_R {
        PSTAT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the PINTENT_F register)."]
    #[inline(always)]
    pub fn pstat7(&self) -> PSTAT7_R {
        PSTAT7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the PINTENT_F register)."]
    #[inline(always)]
    #[must_use]
    pub fn pstat0(&mut self) -> PSTAT0_W<0> {
        PSTAT0_W::new(self)
    }
    #[doc = "Bit 1 - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the PINTENT_F register)."]
    #[inline(always)]
    #[must_use]
    pub fn pstat1(&mut self) -> PSTAT1_W<1> {
        PSTAT1_W::new(self)
    }
    #[doc = "Bit 2 - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the PINTENT_F register)."]
    #[inline(always)]
    #[must_use]
    pub fn pstat2(&mut self) -> PSTAT2_W<2> {
        PSTAT2_W::new(self)
    }
    #[doc = "Bit 3 - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the PINTENT_F register)."]
    #[inline(always)]
    #[must_use]
    pub fn pstat3(&mut self) -> PSTAT3_W<3> {
        PSTAT3_W::new(self)
    }
    #[doc = "Bit 4 - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the PINTENT_F register)."]
    #[inline(always)]
    #[must_use]
    pub fn pstat4(&mut self) -> PSTAT4_W<4> {
        PSTAT4_W::new(self)
    }
    #[doc = "Bit 5 - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the PINTENT_F register)."]
    #[inline(always)]
    #[must_use]
    pub fn pstat5(&mut self) -> PSTAT5_W<5> {
        PSTAT5_W::new(self)
    }
    #[doc = "Bit 6 - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the PINTENT_F register)."]
    #[inline(always)]
    #[must_use]
    pub fn pstat6(&mut self) -> PSTAT6_W<6> {
        PSTAT6_W::new(self)
    }
    #[doc = "Bit 7 - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the PINTENT_F register)."]
    #[inline(always)]
    #[must_use]
    pub fn pstat7(&mut self) -> PSTAT7_W<7> {
        PSTAT7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin Interrupt Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ist](index.html) module"]
pub struct IST_SPEC;
impl crate::RegisterSpec for IST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ist::R](R) reader structure"]
impl crate::Readable for IST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ist::W](W) writer structure"]
impl crate::Writable for IST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IST to value 0"]
impl crate::Resettable for IST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
