#[doc = "Register `PIO0_4` reader"]
pub struct R(crate::R<PIO0_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIO0_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIO0_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIO0_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIO0_4` writer"]
pub struct W(crate::W<PIO0_4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIO0_4_SPEC>;
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
impl From<crate::W<PIO0_4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIO0_4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FUNC` reader - Selects pin function. Values 0x2 to 0x7 are reserved."]
pub type FUNC_R = crate::FieldReader<u8, FUNC_A>;
#[doc = "Selects pin function. Values 0x2 to 0x7 are reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FUNC_A {
    #[doc = "0: PIO0_4 (open-drain pin)."]
    PIO0_4 = 0,
    #[doc = "1: I2C SCL (open-drain pin)."]
    I2C_SCL = 1,
}
impl From<FUNC_A> for u8 {
    #[inline(always)]
    fn from(variant: FUNC_A) -> Self {
        variant as _
    }
}
impl FUNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FUNC_A> {
        match self.bits {
            0 => Some(FUNC_A::PIO0_4),
            1 => Some(FUNC_A::I2C_SCL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PIO0_4`"]
    #[inline(always)]
    pub fn is_pio0_4(&self) -> bool {
        *self == FUNC_A::PIO0_4
    }
    #[doc = "Checks if the value of the field is `I2C_SCL`"]
    #[inline(always)]
    pub fn is_i2c_scl(&self) -> bool {
        *self == FUNC_A::I2C_SCL
    }
}
#[doc = "Field `FUNC` writer - Selects pin function. Values 0x2 to 0x7 are reserved."]
pub type FUNC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PIO0_4_SPEC, u8, FUNC_A, 3, O>;
impl<'a, const O: u8> FUNC_W<'a, O> {
    #[doc = "PIO0_4 (open-drain pin)."]
    #[inline(always)]
    pub fn pio0_4(self) -> &'a mut W {
        self.variant(FUNC_A::PIO0_4)
    }
    #[doc = "I2C SCL (open-drain pin)."]
    #[inline(always)]
    pub fn i2c_scl(self) -> &'a mut W {
        self.variant(FUNC_A::I2C_SCL)
    }
}
#[doc = "Field `I2CMODE` reader - Selects I2C mode (see Section 7.3.8). Select Standard mode (I2CMODE = 00, default) or Standard I/O functionality (I2CMODE = 01) if the pin function is GPIO (FUNC = 000)."]
pub type I2CMODE_R = crate::FieldReader<u8, I2CMODE_A>;
#[doc = "Selects I2C mode (see Section 7.3.8). Select Standard mode (I2CMODE = 00, default) or Standard I/O functionality (I2CMODE = 01) if the pin function is GPIO (FUNC = 000).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2CMODE_A {
    #[doc = "0: Standard mode/ Fast-mode I2C."]
    STANDARD_MODE = 0,
    #[doc = "1: Standard I/O functionality"]
    STANDARD_IO = 1,
    #[doc = "2: Fast-mode Plus I2C"]
    FAST_MODE_PLUS = 2,
}
impl From<I2CMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: I2CMODE_A) -> Self {
        variant as _
    }
}
impl I2CMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2CMODE_A {
        match self.bits {
            0 => I2CMODE_A::STANDARD_MODE,
            1 => I2CMODE_A::STANDARD_IO,
            2 => I2CMODE_A::FAST_MODE_PLUS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD_MODE`"]
    #[inline(always)]
    pub fn is_standard_mode(&self) -> bool {
        *self == I2CMODE_A::STANDARD_MODE
    }
    #[doc = "Checks if the value of the field is `STANDARD_IO`"]
    #[inline(always)]
    pub fn is_standard_io(&self) -> bool {
        *self == I2CMODE_A::STANDARD_IO
    }
    #[doc = "Checks if the value of the field is `FAST_MODE_PLUS`"]
    #[inline(always)]
    pub fn is_fast_mode_plus(&self) -> bool {
        *self == I2CMODE_A::FAST_MODE_PLUS
    }
}
#[doc = "Field `I2CMODE` writer - Selects I2C mode (see Section 7.3.8). Select Standard mode (I2CMODE = 00, default) or Standard I/O functionality (I2CMODE = 01) if the pin function is GPIO (FUNC = 000)."]
pub type I2CMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PIO0_4_SPEC, u8, I2CMODE_A, 2, O>;
impl<'a, const O: u8> I2CMODE_W<'a, O> {
    #[doc = "Standard mode/ Fast-mode I2C."]
    #[inline(always)]
    pub fn standard_mode(self) -> &'a mut W {
        self.variant(I2CMODE_A::STANDARD_MODE)
    }
    #[doc = "Standard I/O functionality"]
    #[inline(always)]
    pub fn standard_io(self) -> &'a mut W {
        self.variant(I2CMODE_A::STANDARD_IO)
    }
    #[doc = "Fast-mode Plus I2C"]
    #[inline(always)]
    pub fn fast_mode_plus(self) -> &'a mut W {
        self.variant(I2CMODE_A::FAST_MODE_PLUS)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function. Values 0x2 to 0x7 are reserved."]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - Selects I2C mode (see Section 7.3.8). Select Standard mode (I2CMODE = 00, default) or Standard I/O functionality (I2CMODE = 01) if the pin function is GPIO (FUNC = 000)."]
    #[inline(always)]
    pub fn i2cmode(&self) -> I2CMODE_R {
        I2CMODE_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function. Values 0x2 to 0x7 are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn func(&mut self) -> FUNC_W<0> {
        FUNC_W::new(self)
    }
    #[doc = "Bits 8:9 - Selects I2C mode (see Section 7.3.8). Select Standard mode (I2CMODE = 00, default) or Standard I/O functionality (I2CMODE = 01) if the pin function is GPIO (FUNC = 000)."]
    #[inline(always)]
    #[must_use]
    pub fn i2cmode(&mut self) -> I2CMODE_W<8> {
        I2CMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O configuration for pin PIO0_4/SCL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_4](index.html) module"]
pub struct PIO0_4_SPEC;
impl crate::RegisterSpec for PIO0_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pio0_4::R](R) reader structure"]
impl crate::Readable for PIO0_4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pio0_4::W](W) writer structure"]
impl crate::Writable for PIO0_4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIO0_4 to value 0x80"]
impl crate::Resettable for PIO0_4_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
