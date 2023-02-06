#![no_std]
#![feature(int_roundings)]
#![allow(incomplete_features)]
#![feature(adt_const_params)]

pub use lpc11uxx as pac;

use portable_atomic as atomic;

pub mod constants;

mod pll_calculator;

#[macro_use]
mod intrinsics;
#[macro_use]
mod peripheral;

pub mod syscon;
pub mod rom_data;
pub mod rom;

extern crate embedded_hal_alpha as eh_alpha;
extern crate embedded_hal_stable as eh_stable;

// In case someone tries to store a static in SRAM1/USBRAM, we enable these banks during pre-init
// NOTE(unsafe) We don't access any static variables, so this should be fine (in theory?)
#[cortex_m_rt::pre_init]
unsafe fn handle_ram_bank_init() {
    use crate::{
        constants::{SRAM1_ENABLED, USBRAM_ENABLED},
        pac::SYSCON,
    };

    if SRAM1_ENABLED {
        (*SYSCON::PTR)
            .sysahbclkctrl
            .modify(|_, w| w.ram1().enabled());
    }
    if USBRAM_ENABLED {
        (*SYSCON::PTR)
            .sysahbclkctrl
            .modify(|_, w| w.usbram().enabled());
    }
}
