#[doc = "Register `MMCTRL` reader"]
pub struct R(crate::R<MMCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMCTRL` writer"]
pub struct W(crate::W<MMCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMCTRL_SPEC>;
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
impl From<crate::W<MMCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MM_ENA` reader - Monitor mode enable."]
pub type MM_ENA_R = crate::BitReader<MM_ENA_A>;
#[doc = "Monitor mode enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MM_ENA_A {
    #[doc = "0: Monitor mode disabled."]
    DISABLED = 0,
    #[doc = "1: The I2C module will enter monitor mode. In this mode the SDA output will be forced high. This will prevent the I2C module from outputting data of any kind (including ACK) onto the I 2C data bus. Depending on the state of the ENA_SCL bit, the output may be also forced high, preventing the module from having control over the I2C clock line."]
    ENABLED = 1,
}
impl From<MM_ENA_A> for bool {
    #[inline(always)]
    fn from(variant: MM_ENA_A) -> Self {
        variant as u8 != 0
    }
}
impl MM_ENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MM_ENA_A {
        match self.bits {
            false => MM_ENA_A::DISABLED,
            true => MM_ENA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MM_ENA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MM_ENA_A::ENABLED
    }
}
#[doc = "Field `MM_ENA` writer - Monitor mode enable."]
pub type MM_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCTRL_SPEC, MM_ENA_A, O>;
impl<'a, const O: u8> MM_ENA_W<'a, O> {
    #[doc = "Monitor mode disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MM_ENA_A::DISABLED)
    }
    #[doc = "The I2C module will enter monitor mode. In this mode the SDA output will be forced high. This will prevent the I2C module from outputting data of any kind (including ACK) onto the I 2C data bus. Depending on the state of the ENA_SCL bit, the output may be also forced high, preventing the module from having control over the I2C clock line."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MM_ENA_A::ENABLED)
    }
}
#[doc = "Field `ENA_SCL` reader - SCL output enable."]
pub type ENA_SCL_R = crate::BitReader<ENA_SCL_A>;
#[doc = "SCL output enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENA_SCL_A {
    #[doc = "0: When this bit is cleared to 0, the SCL output will be forced high when the module is in monitor mode. As described above, this will prevent the module from having any control over the I2C clock line."]
    HIGH = 0,
    #[doc = "1: When this bit is set, the I2C module may exercise the same control over the clock line that it would in normal operation. This means that, acting as a slave peripheral, the I2C module can stretch the clock line (hold it low) until it has had time to respond to an I2C interrupt.\\[1\\]"]
    NORMAL = 1,
}
impl From<ENA_SCL_A> for bool {
    #[inline(always)]
    fn from(variant: ENA_SCL_A) -> Self {
        variant as u8 != 0
    }
}
impl ENA_SCL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA_SCL_A {
        match self.bits {
            false => ENA_SCL_A::HIGH,
            true => ENA_SCL_A::NORMAL,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == ENA_SCL_A::HIGH
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == ENA_SCL_A::NORMAL
    }
}
#[doc = "Field `ENA_SCL` writer - SCL output enable."]
pub type ENA_SCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCTRL_SPEC, ENA_SCL_A, O>;
impl<'a, const O: u8> ENA_SCL_W<'a, O> {
    #[doc = "When this bit is cleared to 0, the SCL output will be forced high when the module is in monitor mode. As described above, this will prevent the module from having any control over the I2C clock line."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(ENA_SCL_A::HIGH)
    }
    #[doc = "When this bit is set, the I2C module may exercise the same control over the clock line that it would in normal operation. This means that, acting as a slave peripheral, the I2C module can stretch the clock line (hold it low) until it has had time to respond to an I2C interrupt.\\[1\\]"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(ENA_SCL_A::NORMAL)
    }
}
#[doc = "Field `MATCH_ALL` reader - Select interrupt register match."]
pub type MATCH_ALL_R = crate::BitReader<MATCH_ALL_A>;
#[doc = "Select interrupt register match.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MATCH_ALL_A {
    #[doc = "0: When this bit is cleared, an interrupt will only be generated when a match occurs to one of the (up-to) four address registers described above. That is, the module will respond as a normal slave as far as address-recognition is concerned."]
    MATCH = 0,
    #[doc = "1: When this bit is set to 1 and the I2C is in monitor mode, an interrupt will be generated on ANY address received. This will enable the part to monitor all traffic on the bus."]
    ANYADDRESS = 1,
}
impl From<MATCH_ALL_A> for bool {
    #[inline(always)]
    fn from(variant: MATCH_ALL_A) -> Self {
        variant as u8 != 0
    }
}
impl MATCH_ALL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MATCH_ALL_A {
        match self.bits {
            false => MATCH_ALL_A::MATCH,
            true => MATCH_ALL_A::ANYADDRESS,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == MATCH_ALL_A::MATCH
    }
    #[doc = "Checks if the value of the field is `ANYADDRESS`"]
    #[inline(always)]
    pub fn is_anyaddress(&self) -> bool {
        *self == MATCH_ALL_A::ANYADDRESS
    }
}
#[doc = "Field `MATCH_ALL` writer - Select interrupt register match."]
pub type MATCH_ALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCTRL_SPEC, MATCH_ALL_A, O>;
impl<'a, const O: u8> MATCH_ALL_W<'a, O> {
    #[doc = "When this bit is cleared, an interrupt will only be generated when a match occurs to one of the (up-to) four address registers described above. That is, the module will respond as a normal slave as far as address-recognition is concerned."]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(MATCH_ALL_A::MATCH)
    }
    #[doc = "When this bit is set to 1 and the I2C is in monitor mode, an interrupt will be generated on ANY address received. This will enable the part to monitor all traffic on the bus."]
    #[inline(always)]
    pub fn anyaddress(self) -> &'a mut W {
        self.variant(MATCH_ALL_A::ANYADDRESS)
    }
}
impl R {
    #[doc = "Bit 0 - Monitor mode enable."]
    #[inline(always)]
    pub fn mm_ena(&self) -> MM_ENA_R {
        MM_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCL output enable."]
    #[inline(always)]
    pub fn ena_scl(&self) -> ENA_SCL_R {
        ENA_SCL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Select interrupt register match."]
    #[inline(always)]
    pub fn match_all(&self) -> MATCH_ALL_R {
        MATCH_ALL_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Monitor mode enable."]
    #[inline(always)]
    #[must_use]
    pub fn mm_ena(&mut self) -> MM_ENA_W<0> {
        MM_ENA_W::new(self)
    }
    #[doc = "Bit 1 - SCL output enable."]
    #[inline(always)]
    #[must_use]
    pub fn ena_scl(&mut self) -> ENA_SCL_W<1> {
        ENA_SCL_W::new(self)
    }
    #[doc = "Bit 2 - Select interrupt register match."]
    #[inline(always)]
    #[must_use]
    pub fn match_all(&mut self) -> MATCH_ALL_W<2> {
        MATCH_ALL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Monitor mode control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmctrl](index.html) module"]
pub struct MMCTRL_SPEC;
impl crate::RegisterSpec for MMCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmctrl::R](R) reader structure"]
impl crate::Readable for MMCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmctrl::W](W) writer structure"]
impl crate::Writable for MMCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMCTRL to value 0"]
impl crate::Resettable for MMCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
