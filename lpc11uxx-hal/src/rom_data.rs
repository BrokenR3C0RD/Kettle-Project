//! Functions from the NXP BootROM.
//!
//! These are implemented mainly for HAL code to use. You likely don't want to use any of this.
//!
//! If you're certain you want to use them, however, please check out the NXP LPC11Uxx Reference Manual for usage.

use crate::atomic::{AtomicUsize, Ordering};
pub use lpc_usbd_rom as usbd;
use crate::{intrinsics, intrinsics_aliases};

type IapFunction = extern "C" fn(*const u32, *mut u32) -> ();
const IAP_LOCATION: *const IapFunction = 0x1FFF_1FF1 as _;

/// A thunk for the IAP entry function.
///
/// You likely don't want to use this. If you're sure you do, see the Reference Manual for usage.
#[inline(always)]
pub unsafe fn iap_entry(command_param: &[u32], status_result: &mut [u32]) {
    assert!(command_param.len() >= 5);
    assert!(status_result.len() >= 4);

    let func: IapFunction = core::mem::transmute(IAP_LOCATION);
    func(command_param.as_ptr(), status_result.as_mut_ptr())
}

/// The ROM driver table provides access to the different functions provided by the BootROM.
///
/// You likely don't want to use this. If you're sure you do, see the Reference Manual for usage.
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
    /// Gets the pointer to the [RomDriverTable].
    pub const PTR: *const *const RomDriverTable = 0x1FFF_1FF8 as _;

    #[inline(always)]
    pub const fn ptr() -> *const RomDriverTable {
        unsafe { *Self::PTR }
    }

    #[inline(always)]
    pub fn usb() -> *const usbd::USBD_API_T {
        unsafe { (*Self::ptr())._usb }
    }

    #[inline(always)]
    pub fn power() -> *const PowerApiTable {
        unsafe { (*Self::ptr())._power }
    }

    pub fn romdiv() -> *const RomDivTable {
        unsafe { (*Self::ptr())._romdiv }
    }
}

/// Provides access to the ROM Power API.
///
/// You likely don't want to use this. If you're sure you do, see the Reference Manual for usage.
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

/// Provdes access to the ROM division intrinsics.
///
/// You likely don't want to use this. If you're sure you do, see the Reference Manual for usage.

#[repr(C)]
pub struct RomDivTable {
    pub(crate) sidiv: *const extern "C" fn(i32, i32) -> i32,
    pub(crate) uidiv: *const extern "C" fn(u32, u32) -> u32,
    pub(crate) sidivmod: *const extern "C" fn(i32, i32) -> DivReturn<i32>,
    pub(crate) uidivmod: *const extern "C" fn(u32, u32) -> DivReturn<u32>,
}

/// Creates thunks to the ROM integer division intrinsics which cache the function pointers.
macro_rules! cached_integer_intrinsics {
    (
        $(
            $(#[$outer:meta])*
            fn $name:ident ($a:ty, $b:ty) -> $c:ty
        );+;
        ) => {
        $(
            $(#[$outer])*
            #[inline(always)]
            pub fn $name (num: $a, den: $b) -> $c {
                type Func = extern "C" fn(num: $a, den: $b) -> $c;

                // Integer division is probably hot, so we cache the pointer.
                static CACHED_PTR: AtomicUsize = AtomicUsize::new(0);

                let p: *const Func = match CACHED_PTR.load(Ordering::Relaxed) as usize {
                    0 => {
                        let ptr = unsafe { (*RomDriverTable::romdiv()).$name };
                        CACHED_PTR.store(ptr as usize, Ordering::Relaxed);
                        ptr
                    },
                    val => {
                        val as *const Func
                    }
                };

                unsafe {
                    let func: Func = core::mem::transmute(p);

                    func(num, den)
                }
            }
        )+
    };
}

cached_integer_intrinsics!(
    /// Performs signed integer division using the ROM intrinsics.
    fn sidiv(i32, i32) -> i32;
    /// Performs unsigned integer division using the ROM intrinsics.
    fn uidiv(u32, u32) -> u32;
    /// Performs signed integer division with remainder using the ROM intrinsics.
    fn sidivmod(i32, i32) -> DivReturn<i32>;
    /// Performs unsigned integer division with remainder using the ROM intrinsics.
    fn uidivmod(u32, u32) -> DivReturn<u32>;
);

// The LPC bootrom provides built-in division routines, so we alias those to intrinsics
// This, in theory, will save some memory

macro_rules! integer_intrinsics {
    (
        $div_fn:ident,
        $mod_fn:ident,
        $divmod_fn:ident,
        $aeabi_div_alias:ident,
        $aeabi_divmod_alias:ident,
        $aeabi_divmod_ret:ty,
        $ty:ty,
        $idiv_fn:ident,
        $idivmod_fn:ident
    ) => {
        intrinsics! (
            #[aeabi = $aeabi_div_alias]
            extern "C" fn $div_fn(n: $ty, d: $ty) -> $ty {
                $idiv_fn(n, d)
            }

            extern "C" fn $mod_fn(n: $ty, d: $ty) -> $ty {
                $idivmod_fn(n, d).rem
            }

            extern "C" fn $divmod_fn(n: $ty, d: $ty, rem: Option<&mut $ty>) -> $ty {
                let quot_rem = $idivmod_fn(n, d);

                if let Some(rem) = rem {
                    *rem = quot_rem.rem;
                }

                quot_rem.quot
            }

            extern "aapcs" fn $aeabi_divmod_alias(n: $ty, d: $ty) -> $aeabi_divmod_ret {
                $idivmod_fn(n, d)
            }
        );
    };
}

integer_intrinsics!(
    __udivsi3,
    __umodsi3,
    __udivmodsi4,
    __aeabi_uidiv,
    __aeabi_uidivmod,
    crate::rom_data::DivReturn<u32>,
    u32,
    uidiv,
    uidivmod
);

integer_intrinsics!(
    __divsi3,
    __modsi3,
    __divmodsi4,
    __aeabi_idiv,
    __aeabi_idivmod,
    crate::rom_data::DivReturn<i32>,
    i32,
    sidiv,
    sidivmod
);
