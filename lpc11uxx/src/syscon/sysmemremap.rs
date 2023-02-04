#[doc = "Register `SYSMEMREMAP` reader"]
pub struct R(crate::R<SYSMEMREMAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSMEMREMAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSMEMREMAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSMEMREMAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSMEMREMAP` writer"]
pub struct W(crate::W<SYSMEMREMAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSMEMREMAP_SPEC>;
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
impl From<crate::W<SYSMEMREMAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSMEMREMAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAP` reader - System memory remap. Value 0x3 is reserved."]
pub type MAP_R = crate::FieldReader<u8, MAP_A>;
#[doc = "System memory remap. Value 0x3 is reserved.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MAP_A {
    #[doc = "0: Boot Loader Mode. Interrupt vectors are re-mapped to Boot ROM."]
    BOOT_LOADER = 0,
    #[doc = "1: User RAM Mode. Interrupt vectors are re-mapped to Static RAM."]
    USER_RAM = 1,
    #[doc = "2: User Flash Mode. Interrupt vectors are not re-mapped and reside in Flash."]
    USER_FLASH = 2,
}
impl From<MAP_A> for u8 {
    #[inline(always)]
    fn from(variant: MAP_A) -> Self {
        variant as _
    }
}
impl MAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MAP_A> {
        match self.bits {
            0 => Some(MAP_A::BOOT_LOADER),
            1 => Some(MAP_A::USER_RAM),
            2 => Some(MAP_A::USER_FLASH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BOOT_LOADER`"]
    #[inline(always)]
    pub fn is_boot_loader(&self) -> bool {
        *self == MAP_A::BOOT_LOADER
    }
    #[doc = "Checks if the value of the field is `USER_RAM`"]
    #[inline(always)]
    pub fn is_user_ram(&self) -> bool {
        *self == MAP_A::USER_RAM
    }
    #[doc = "Checks if the value of the field is `USER_FLASH`"]
    #[inline(always)]
    pub fn is_user_flash(&self) -> bool {
        *self == MAP_A::USER_FLASH
    }
}
#[doc = "Field `MAP` writer - System memory remap. Value 0x3 is reserved."]
pub type MAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSMEMREMAP_SPEC, u8, MAP_A, 2, O>;
impl<'a, const O: u8> MAP_W<'a, O> {
    #[doc = "Boot Loader Mode. Interrupt vectors are re-mapped to Boot ROM."]
    #[inline(always)]
    pub fn boot_loader(self) -> &'a mut W {
        self.variant(MAP_A::BOOT_LOADER)
    }
    #[doc = "User RAM Mode. Interrupt vectors are re-mapped to Static RAM."]
    #[inline(always)]
    pub fn user_ram(self) -> &'a mut W {
        self.variant(MAP_A::USER_RAM)
    }
    #[doc = "User Flash Mode. Interrupt vectors are not re-mapped and reside in Flash."]
    #[inline(always)]
    pub fn user_flash(self) -> &'a mut W {
        self.variant(MAP_A::USER_FLASH)
    }
}
impl R {
    #[doc = "Bits 0:1 - System memory remap. Value 0x3 is reserved."]
    #[inline(always)]
    pub fn map(&self) -> MAP_R {
        MAP_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - System memory remap. Value 0x3 is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn map(&mut self) -> MAP_W<0> {
        MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System memory remap\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysmemremap](index.html) module"]
pub struct SYSMEMREMAP_SPEC;
impl crate::RegisterSpec for SYSMEMREMAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysmemremap::R](R) reader structure"]
impl crate::Readable for SYSMEMREMAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysmemremap::W](W) writer structure"]
impl crate::Writable for SYSMEMREMAP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSMEMREMAP to value 0x02"]
impl crate::Resettable for SYSMEMREMAP_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
