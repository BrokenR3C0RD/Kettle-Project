#[doc = "Register `EMR` reader"]
pub struct R(crate::R<EMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMR` writer"]
pub struct W(crate::W<EMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMR_SPEC>;
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
impl From<crate::W<EMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EM0` reader - External Match 0. This bit reflects the state of output CT32Bn_MAT0, whether or not this output is connected to its pin. When a match occurs between the TC and MR0, this bit can either toggle, go LOW, go HIGH, or do nothing. Bits EMR\\[5:4\\]
control the functionality of this output. This bit is driven to the CT32B0_MAT0/CT32B1_MAT0 pins if the match function is selected in the IOCON registers (0 = LOW, 1 = HIGH)."]
pub type EM0_R = crate::BitReader<bool>;
#[doc = "Field `EM0` writer - External Match 0. This bit reflects the state of output CT32Bn_MAT0, whether or not this output is connected to its pin. When a match occurs between the TC and MR0, this bit can either toggle, go LOW, go HIGH, or do nothing. Bits EMR\\[5:4\\]
control the functionality of this output. This bit is driven to the CT32B0_MAT0/CT32B1_MAT0 pins if the match function is selected in the IOCON registers (0 = LOW, 1 = HIGH)."]
pub type EM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR_SPEC, bool, O>;
#[doc = "Field `EM1` reader - External Match 1. This bit reflects the state of output CT32Bn_MAT1, whether or not this output is connected to its pin. When a match occurs between the TC and MR1, this bit can either toggle, go LOW, go HIGH, or do nothing. Bits EMR\\[7:6\\]
control the functionality of this output. This bit is driven to the CT32B0_MAT1/CT32B1_MAT1 pins if the match function is selected in the IOCON registers (0 = LOW, 1 = HIGH)."]
pub type EM1_R = crate::BitReader<bool>;
#[doc = "Field `EM1` writer - External Match 1. This bit reflects the state of output CT32Bn_MAT1, whether or not this output is connected to its pin. When a match occurs between the TC and MR1, this bit can either toggle, go LOW, go HIGH, or do nothing. Bits EMR\\[7:6\\]
control the functionality of this output. This bit is driven to the CT32B0_MAT1/CT32B1_MAT1 pins if the match function is selected in the IOCON registers (0 = LOW, 1 = HIGH)."]
pub type EM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR_SPEC, bool, O>;
#[doc = "Field `EM2` reader - External Match 2. This bit reflects the state of output CT32Bn_MAT2, whether or not this output is connected to its pin. When a match occurs between the TC and MR2, this bit can either toggle, go LOW, go HIGH, or do nothing. Bits EMR\\[9:8\\]
control the functionality of this output. This bit is driven to the CT32B0_MAT2/CT32B1_MAT2 pins if the match function is selected in the IOCON registers (0 = LOW, 1 = HIGH)."]
pub type EM2_R = crate::BitReader<bool>;
#[doc = "Field `EM2` writer - External Match 2. This bit reflects the state of output CT32Bn_MAT2, whether or not this output is connected to its pin. When a match occurs between the TC and MR2, this bit can either toggle, go LOW, go HIGH, or do nothing. Bits EMR\\[9:8\\]
control the functionality of this output. This bit is driven to the CT32B0_MAT2/CT32B1_MAT2 pins if the match function is selected in the IOCON registers (0 = LOW, 1 = HIGH)."]
pub type EM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR_SPEC, bool, O>;
#[doc = "Field `EM3` reader - External Match 3. This bit reflects the state of output CT32Bn_MAT3, whether or not this output is connected to its pin. When a match occurs between the TC and MR3, this bit can either toggle, go LOW, go HIGH, or do nothing. Bits EMR\\[11:10\\]
control the functionality of this output. This bit is driven to the CT32B3_MAT0/CT32B1_MAT3 pins if the match function is selected in the IOCON registers (0 = LOW, 1 = HIGH)."]
pub type EM3_R = crate::BitReader<bool>;
#[doc = "Field `EM3` writer - External Match 3. This bit reflects the state of output CT32Bn_MAT3, whether or not this output is connected to its pin. When a match occurs between the TC and MR3, this bit can either toggle, go LOW, go HIGH, or do nothing. Bits EMR\\[11:10\\]
control the functionality of this output. This bit is driven to the CT32B3_MAT0/CT32B1_MAT3 pins if the match function is selected in the IOCON registers (0 = LOW, 1 = HIGH)."]
pub type EM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR_SPEC, bool, O>;
#[doc = "Field `EMC0` reader - External Match Control 0. Determines the functionality of External Match 0."]
pub type EMC0_R = crate::FieldReader<u8, EMC0_A>;
#[doc = "External Match Control 0. Determines the functionality of External Match 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EMC0_A {
    #[doc = "0: Do Nothing."]
    DO_NOTHING = 0,
    #[doc = "1: Clear the corresponding External Match bit/output to 0 (CT32Bi_MAT0 pin is LOW if pinned out)."]
    CLEAR = 1,
    #[doc = "2: Set the corresponding External Match bit/output to 1 (CT32Bi_MAT0 pin is HIGH if pinned out)."]
    SET = 2,
    #[doc = "3: Toggle the corresponding External Match bit/output."]
    TOGGLE = 3,
}
impl From<EMC0_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC0_A) -> Self {
        variant as _
    }
}
impl EMC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMC0_A {
        match self.bits {
            0 => EMC0_A::DO_NOTHING,
            1 => EMC0_A::CLEAR,
            2 => EMC0_A::SET,
            3 => EMC0_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DO_NOTHING`"]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        *self == EMC0_A::DO_NOTHING
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == EMC0_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == EMC0_A::SET
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == EMC0_A::TOGGLE
    }
}
#[doc = "Field `EMC0` writer - External Match Control 0. Determines the functionality of External Match 0."]
pub type EMC0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, EMR_SPEC, u8, EMC0_A, 2, O>;
impl<'a, const O: u8> EMC0_W<'a, O> {
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut W {
        self.variant(EMC0_A::DO_NOTHING)
    }
    #[doc = "Clear the corresponding External Match bit/output to 0 (CT32Bi_MAT0 pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EMC0_A::CLEAR)
    }
    #[doc = "Set the corresponding External Match bit/output to 1 (CT32Bi_MAT0 pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(EMC0_A::SET)
    }
    #[doc = "Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC0_A::TOGGLE)
    }
}
#[doc = "Field `EMC1` reader - External Match Control 1. Determines the functionality of External Match 1."]
pub type EMC1_R = crate::FieldReader<u8, EMC1_A>;
#[doc = "External Match Control 1. Determines the functionality of External Match 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EMC1_A {
    #[doc = "0: Do Nothing."]
    DO_NOTHING = 0,
    #[doc = "1: Clear the corresponding External Match bit/output to 0 (CT32Bi_MAT1 pin is LOW if pinned out)."]
    CLEAR = 1,
    #[doc = "2: Set the corresponding External Match bit/output to 1 (CT32Bi_MAT1 pin is HIGH if pinned out)."]
    SET = 2,
    #[doc = "3: Toggle the corresponding External Match bit/output."]
    TOGGLE = 3,
}
impl From<EMC1_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC1_A) -> Self {
        variant as _
    }
}
impl EMC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMC1_A {
        match self.bits {
            0 => EMC1_A::DO_NOTHING,
            1 => EMC1_A::CLEAR,
            2 => EMC1_A::SET,
            3 => EMC1_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DO_NOTHING`"]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        *self == EMC1_A::DO_NOTHING
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == EMC1_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == EMC1_A::SET
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == EMC1_A::TOGGLE
    }
}
#[doc = "Field `EMC1` writer - External Match Control 1. Determines the functionality of External Match 1."]
pub type EMC1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, EMR_SPEC, u8, EMC1_A, 2, O>;
impl<'a, const O: u8> EMC1_W<'a, O> {
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut W {
        self.variant(EMC1_A::DO_NOTHING)
    }
    #[doc = "Clear the corresponding External Match bit/output to 0 (CT32Bi_MAT1 pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EMC1_A::CLEAR)
    }
    #[doc = "Set the corresponding External Match bit/output to 1 (CT32Bi_MAT1 pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(EMC1_A::SET)
    }
    #[doc = "Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC1_A::TOGGLE)
    }
}
#[doc = "Field `EMC2` reader - External Match Control 2. Determines the functionality of External Match 2."]
pub type EMC2_R = crate::FieldReader<u8, EMC2_A>;
#[doc = "External Match Control 2. Determines the functionality of External Match 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EMC2_A {
    #[doc = "0: Do Nothing."]
    DO_NOTHING = 0,
    #[doc = "1: Clear the corresponding External Match bit/output to 0 (CT32Bi_MAT2 pin is LOW if pinned out)."]
    CLEAR = 1,
    #[doc = "2: Set the corresponding External Match bit/output to 1 (CT32Bi_MAT2 pin is HIGH if pinned out)."]
    SET = 2,
    #[doc = "3: Toggle the corresponding External Match bit/output."]
    TOGGLE = 3,
}
impl From<EMC2_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC2_A) -> Self {
        variant as _
    }
}
impl EMC2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMC2_A {
        match self.bits {
            0 => EMC2_A::DO_NOTHING,
            1 => EMC2_A::CLEAR,
            2 => EMC2_A::SET,
            3 => EMC2_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DO_NOTHING`"]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        *self == EMC2_A::DO_NOTHING
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == EMC2_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == EMC2_A::SET
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == EMC2_A::TOGGLE
    }
}
#[doc = "Field `EMC2` writer - External Match Control 2. Determines the functionality of External Match 2."]
pub type EMC2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, EMR_SPEC, u8, EMC2_A, 2, O>;
impl<'a, const O: u8> EMC2_W<'a, O> {
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut W {
        self.variant(EMC2_A::DO_NOTHING)
    }
    #[doc = "Clear the corresponding External Match bit/output to 0 (CT32Bi_MAT2 pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EMC2_A::CLEAR)
    }
    #[doc = "Set the corresponding External Match bit/output to 1 (CT32Bi_MAT2 pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(EMC2_A::SET)
    }
    #[doc = "Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC2_A::TOGGLE)
    }
}
#[doc = "Field `EMC3` reader - External Match Control 3. Determines the functionality of External Match 3."]
pub type EMC3_R = crate::FieldReader<u8, EMC3_A>;
#[doc = "External Match Control 3. Determines the functionality of External Match 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EMC3_A {
    #[doc = "0: Do Nothing."]
    DO_NOTHING = 0,
    #[doc = "1: Clear the corresponding External Match bit/output to 0 (CT32Bi_MAT3 pin is LOW if pinned out)."]
    CLEAR = 1,
    #[doc = "2: Set the corresponding External Match bit/output to 1 (CT32Bi_MAT3 pin is HIGH if pinned out)."]
    SET = 2,
    #[doc = "3: Toggle the corresponding External Match bit/output."]
    TOGGLE = 3,
}
impl From<EMC3_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC3_A) -> Self {
        variant as _
    }
}
impl EMC3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMC3_A {
        match self.bits {
            0 => EMC3_A::DO_NOTHING,
            1 => EMC3_A::CLEAR,
            2 => EMC3_A::SET,
            3 => EMC3_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DO_NOTHING`"]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        *self == EMC3_A::DO_NOTHING
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == EMC3_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == EMC3_A::SET
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == EMC3_A::TOGGLE
    }
}
#[doc = "Field `EMC3` writer - External Match Control 3. Determines the functionality of External Match 3."]
pub type EMC3_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, EMR_SPEC, u8, EMC3_A, 2, O>;
impl<'a, const O: u8> EMC3_W<'a, O> {
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut W {
        self.variant(EMC3_A::DO_NOTHING)
    }
    #[doc = "Clear the corresponding External Match bit/output to 0 (CT32Bi_MAT3 pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EMC3_A::CLEAR)
    }
    #[doc = "Set the corresponding External Match bit/output to 1 (CT32Bi_MAT3 pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(EMC3_A::SET)
    }
    #[doc = "Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC3_A::TOGGLE)
    }
}
impl R {
    #[doc = "Bit 0 - External Match 0. This bit reflects the state of output CT32Bn_MAT0, whether or not this output is connected to its pin. When a match occurs between the TC and MR0, this bit can either toggle, go LOW, go HIGH, or do nothing. Bits EMR\\[5:4\\]
control the functionality of this output. This bit is driven to the CT32B0_MAT0/CT32B1_MAT0 pins if the match function is selected in the IOCON registers (0 = LOW, 1 = HIGH)."]
    #[inline(always)]
    pub fn em0(&self) -> EM0_R {
        EM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Match 1. This bit reflects the state of output CT32Bn_MAT1, whether or not this output is connected to its pin. When a match occurs between the TC and MR1, this bit can either toggle, go LOW, go HIGH, or do nothing. Bits EMR\\[7:6\\]
control the functionality of this output. This bit is driven to the CT32B0_MAT1/CT32B1_MAT1 pins if the match function is selected in the IOCON registers (0 = LOW, 1 = HIGH)."]
    #[inline(always)]
    pub fn em1(&self) -> EM1_R {
        EM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Match 2. This bit reflects the state of output CT32Bn_MAT2, whether or not this output is connected to its pin. When a match occurs between the TC and MR2, this bit can either toggle, go LOW, go HIGH, or do nothing. Bits EMR\\[9:8\\]
control the functionality of this output. This bit is driven to the CT32B0_MAT2/CT32B1_MAT2 pins if the match function is selected in the IOCON registers (0 = LOW, 1 = HIGH)."]
    #[inline(always)]
    pub fn em2(&self) -> EM2_R {
        EM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External Match 3. This bit reflects the state of output CT32Bn_MAT3, whether or not this output is connected to its pin. When a match occurs between the TC and MR3, this bit can either toggle, go LOW, go HIGH, or do nothing. Bits EMR\\[11:10\\]
control the functionality of this output. This bit is driven to the CT32B3_MAT0/CT32B1_MAT3 pins if the match function is selected in the IOCON registers (0 = LOW, 1 = HIGH)."]
    #[inline(always)]
    pub fn em3(&self) -> EM3_R {
        EM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - External Match Control 0. Determines the functionality of External Match 0."]
    #[inline(always)]
    pub fn emc0(&self) -> EMC0_R {
        EMC0_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - External Match Control 1. Determines the functionality of External Match 1."]
    #[inline(always)]
    pub fn emc1(&self) -> EMC1_R {
        EMC1_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - External Match Control 2. Determines the functionality of External Match 2."]
    #[inline(always)]
    pub fn emc2(&self) -> EMC2_R {
        EMC2_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - External Match Control 3. Determines the functionality of External Match 3."]
    #[inline(always)]
    pub fn emc3(&self) -> EMC3_R {
        EMC3_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - External Match 0. This bit reflects the state of output CT32Bn_MAT0, whether or not this output is connected to its pin. When a match occurs between the TC and MR0, this bit can either toggle, go LOW, go HIGH, or do nothing. Bits EMR\\[5:4\\]
control the functionality of this output. This bit is driven to the CT32B0_MAT0/CT32B1_MAT0 pins if the match function is selected in the IOCON registers (0 = LOW, 1 = HIGH)."]
    #[inline(always)]
    #[must_use]
    pub fn em0(&mut self) -> EM0_W<0> {
        EM0_W::new(self)
    }
    #[doc = "Bit 1 - External Match 1. This bit reflects the state of output CT32Bn_MAT1, whether or not this output is connected to its pin. When a match occurs between the TC and MR1, this bit can either toggle, go LOW, go HIGH, or do nothing. Bits EMR\\[7:6\\]
control the functionality of this output. This bit is driven to the CT32B0_MAT1/CT32B1_MAT1 pins if the match function is selected in the IOCON registers (0 = LOW, 1 = HIGH)."]
    #[inline(always)]
    #[must_use]
    pub fn em1(&mut self) -> EM1_W<1> {
        EM1_W::new(self)
    }
    #[doc = "Bit 2 - External Match 2. This bit reflects the state of output CT32Bn_MAT2, whether or not this output is connected to its pin. When a match occurs between the TC and MR2, this bit can either toggle, go LOW, go HIGH, or do nothing. Bits EMR\\[9:8\\]
control the functionality of this output. This bit is driven to the CT32B0_MAT2/CT32B1_MAT2 pins if the match function is selected in the IOCON registers (0 = LOW, 1 = HIGH)."]
    #[inline(always)]
    #[must_use]
    pub fn em2(&mut self) -> EM2_W<2> {
        EM2_W::new(self)
    }
    #[doc = "Bit 3 - External Match 3. This bit reflects the state of output CT32Bn_MAT3, whether or not this output is connected to its pin. When a match occurs between the TC and MR3, this bit can either toggle, go LOW, go HIGH, or do nothing. Bits EMR\\[11:10\\]
control the functionality of this output. This bit is driven to the CT32B3_MAT0/CT32B1_MAT3 pins if the match function is selected in the IOCON registers (0 = LOW, 1 = HIGH)."]
    #[inline(always)]
    #[must_use]
    pub fn em3(&mut self) -> EM3_W<3> {
        EM3_W::new(self)
    }
    #[doc = "Bits 4:5 - External Match Control 0. Determines the functionality of External Match 0."]
    #[inline(always)]
    #[must_use]
    pub fn emc0(&mut self) -> EMC0_W<4> {
        EMC0_W::new(self)
    }
    #[doc = "Bits 6:7 - External Match Control 1. Determines the functionality of External Match 1."]
    #[inline(always)]
    #[must_use]
    pub fn emc1(&mut self) -> EMC1_W<6> {
        EMC1_W::new(self)
    }
    #[doc = "Bits 8:9 - External Match Control 2. Determines the functionality of External Match 2."]
    #[inline(always)]
    #[must_use]
    pub fn emc2(&mut self) -> EMC2_W<8> {
        EMC2_W::new(self)
    }
    #[doc = "Bits 10:11 - External Match Control 3. Determines the functionality of External Match 3."]
    #[inline(always)]
    #[must_use]
    pub fn emc3(&mut self) -> EMC3_W<10> {
        EMC3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Match Register. The EMR controls the match function and the external match pins CT32Bn_MAT\\[3:0\\].\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emr](index.html) module"]
pub struct EMR_SPEC;
impl crate::RegisterSpec for EMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emr::R](R) reader structure"]
impl crate::Readable for EMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emr::W](W) writer structure"]
impl crate::Writable for EMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EMR to value 0"]
impl crate::Resettable for EMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
