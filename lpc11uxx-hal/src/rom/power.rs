use embedded_time::rate::Kilohertz;

use crate::rom_data::{RomDriverTable, PowerApiTable};

pub enum PllFreqMode {
    Equal = 0,
    LessThanEqual = 1,
    GreaterThanEqual = 2,
    Approx = 3
}

pub enum PowerMode {
    Default = 0,
    Performance = 1,
    Balanced = 2,
    LowPower = 3,
}

/// This is a safe wrapper for [PowerApiTable.set_pll].
/// > This routine sets up the system PLL according to the calling arguments. If the expected
/// > clock can be obtained by simply dividing the system PLL input, set_pll bypasses the PLL
/// > to lower system power consumption.
/// 
/// Note that this is unsafe because it changes clock information, which could result in undefined behavior
/// from peripherals depending on it.
pub unsafe fn configure_pll(sysosc: Kilohertz, target: Kilohertz, mode: PllFreqMode, timeout: u32) {
    
}