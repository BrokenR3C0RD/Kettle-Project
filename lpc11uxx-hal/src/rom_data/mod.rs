//! Functions from the NXP BootROM.

pub use lpc_usbd_rom as usbd;

type IapFunction = extern "C" fn(*const u32, *mut u32) -> ();
const IAP_LOCATION: *const IapFunction = 0x1FFF_1FF1 as _;

#[inline(always)]
pub unsafe fn iap_entry(command_param: &[u32], status_result: &mut [u32]) {
    assert!(command_param.len() >= 5);
    assert!(status_result.len() >= 4);
    (*IAP_LOCATION)(command_param.as_ptr(), status_result.as_mut_ptr())
}

#[repr(C)]
pub struct RomDriverTable {
    _usb: *const usbd::USBD_API_T,
    _dt1: *const (),
    _dt2: *const (),
    _power: *const PowerApiTable,
    _romdiv: *const RomDivTable,
}

unsafe impl Sync for RomDriverTable {}

impl RomDriverTable {
    pub const PTR: *const RomDriverTable = 0x1FFF_1FF8 as _;

    #[inline(always)]
    pub const fn ptr() -> *const RomDriverTable {
        Self::PTR
    }

    #[inline(always)]
    pub const fn usb() -> *const usbd::USBD_API_T {
        unsafe { (*Self::PTR)._usb }
    }
    
    #[inline(always)]
    pub const fn power() -> *const PowerApiTable {
        unsafe { (*Self::PTR)._power }
    }

    #[inline(always)]
    pub const fn romdiv() -> *const RomDivTable {
        unsafe { (*Self::PTR)._romdiv }
    }
}

#[repr(C)]
pub struct PowerApiTable {
    _set_pll: *const extern "C" fn(*const u32, *mut u32),
    _set_power: *const extern "C" fn(*const u32, *mut u32),
}

impl PowerApiTable {
    #[inline(always)]
    pub unsafe fn set_pll(&self, cmd: &[u32], resp: &mut [u32]) -> () {
        assert!(cmd.len() >= 4);
        assert!(resp.len() >= 2);

        (*self._set_pll)(cmd.as_ptr(), resp.as_mut_ptr());
    }

    #[inline(always)]
    pub unsafe fn set_power(&self, cmd: &[u32], resp: &mut [u32]) -> () {
        assert!(cmd.len() >= 3);
        assert!(resp.len() >= 1);

        (*self._set_power)(cmd.as_ptr(), resp.as_mut_ptr());
    }
}

#[repr(C)]
pub struct DivReturn<T> {
    pub quot: T,
    pub rem: T,
}

#[repr(C)]
pub struct RomDivTable {
    _sidiv: *const extern "C" fn(i32, i32) -> i32,
    _uidiv: *const extern "C" fn(u32, u32) -> u32,
    _sidivmod: *const extern "C" fn(i32, i32) -> DivReturn<i32>,
    _uidivmod: *const extern "C" fn(u32, u32) -> DivReturn<u32>,
}

impl RomDivTable {
    #[inline(always)]
    pub fn sidiv(&self, numerator: i32, denominator: i32) -> i32 {
        unsafe { (*self._sidiv)(numerator, denominator) }
    }

    #[inline(always)]
    pub fn uidiv(&self, numerator: u32, denominator: u32) -> u32 {
        unsafe { (*self._uidiv)(numerator, denominator) }
    }

    #[inline(always)]
    pub fn sidivmod(&self, numerator: i32, denominator: i32) -> DivReturn<i32> {
        unsafe { (*self._sidivmod)(numerator, denominator) }
    }

    #[inline(always)]
    pub fn uidivmod(&self, numerator: u32, denominator: u32) -> DivReturn<u32> {
        unsafe { (*self._uidivmod)(numerator, denominator) }
    }
}
