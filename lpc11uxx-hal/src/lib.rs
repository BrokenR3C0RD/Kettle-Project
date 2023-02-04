#![no_std]

use lpc11uxx::SYSCON;
pub mod constants;
pub mod rom_data;

#[macro_use]
mod intrinsics;

extern crate embedded_hal_stable as eh_stable;
extern crate embedded_hal_alpha as eh_alpha;


// In case someone tries to store a static in SRAM1/USBRAM, we enable these banks during pre-init
// NOTE(unsafe) We don't access any static variables, so this should be fine (in theory?)
#[cortex_m_rt::pre_init]
unsafe fn handle_ram_bank_init() {
    if constants::SRAM1_ENABLED {
        (*SYSCON::PTR)
            .sysahbclkctrl
            .modify(|_, w| w.ram1().enabled());
    }
    if constants::USBRAM_ENABLED {
        (*SYSCON::PTR)
            .sysahbclkctrl
            .modify(|_, w| w.usbram().enabled());
    }
}
