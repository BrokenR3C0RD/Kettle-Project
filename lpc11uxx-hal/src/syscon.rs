use core::{marker::PhantomData, num::TryFromIntError};

use crate::pac::{FLASHCTRL, SYSCON};
use embedded_time::rate::Hertz;

#[cfg(feature = "defmt")]
use defmt::Format;

pub use crate::pac::syscon::wdtoscctrl::FREQSEL_A as WwdtOscFreq;

const IRC_FREQ: Hertz = Hertz(12_000_000_u32);

trait WwdtOscFreqExt {
    fn frequency(&self) -> Hertz;
}

impl WwdtOscFreqExt for WwdtOscFreq {
    #[inline(always)]
    fn frequency(&self) -> Hertz {
        match self {
            WwdtOscFreq::_0_6_MHZ => Hertz(600_000_u32),
            WwdtOscFreq::_1_05_MHZ => Hertz(1_050_000_u32),
            WwdtOscFreq::_1_4_MHZ => Hertz(1_400_000_u32),
            WwdtOscFreq::_1_75_MHZ => Hertz(1_750_000_u32),
            WwdtOscFreq::_2_1_MHZ => Hertz(2_100_000_u32),
            WwdtOscFreq::_2_4_MHZ => Hertz(2_400_000_u32),
            WwdtOscFreq::_2_7_MHZ => Hertz(2_700_000_u32),
            WwdtOscFreq::_3_0_MHZ => Hertz(3_000_000_u32),
            WwdtOscFreq::_3_25_MHZ => Hertz(3_250_000_u32),
            WwdtOscFreq::_3_5_MHZ => Hertz(3_500_000_u32),
            WwdtOscFreq::_3_75_MHZ => Hertz(3_750_000_u32),
            WwdtOscFreq::_4_0_MHZ => Hertz(4_000_000_u32),
            WwdtOscFreq::_4_2_MHZ => Hertz(4_200_000_u32),
            WwdtOscFreq::_4_4_MHZ => Hertz(4_400_000_u32),
            WwdtOscFreq::_4_6_MHZ => Hertz(4_600_000_u32),
        }
    }
}

pub trait SysconExt {
    fn constrain(self) -> Syscon;
}

impl SysconExt for SYSCON {
    #[inline(always)]
    fn constrain(self) -> Syscon {
        Syscon {
            _private: self,
            clock_config: unsafe { ClockConfig::new() },
        }
    }
}

#[derive(Clone, Debug)]
pub enum PllClockSource {
    Irc,
    SysOsc(Hertz),
}

impl PllClockSource {
    #[inline(always)]
    pub fn frequency(&self) -> Hertz {
        match self {
            PllClockSource::Irc => IRC_FREQ,
            PllClockSource::SysOsc(hz) => hz.clone(),
        }
    }
}

/// Provides options for configuring the main clock manually.
///
/// This escape hatch of sorts allows for fine-tuned control over the parameters used for calculating the main clock
/// source and its dividers.
#[derive(Clone, Debug)]
pub enum MainClockSourceInfo {
    /// Use the IRC oscillator as the main clock. This runs at a fixed 12 MHz.
    Irc,

    /// Use the system oscillator, running at the provided [Hertz], directly as the main clock.
    SysOsc(Hertz),

    /// Use the watchdog oscillator, running at the provided [WwdtOscFreq] and with the specified divider (which is a multiple of 2 between 2 and 64), as the main clock.
    ///
    /// Note that although the watchdog oscillator uses the least power, it is also the least accurate (F_clkana can be +/- 40% of
    /// the listed frequency value).
    WwdtOsc(WwdtOscFreq, u8),

    /// Use the System PLL, with the specified [PllClockSource], M and P values as the main clock.
    ///
    /// Please ensure that your values meet the requirements specified in the Reference Manual, section 3.10.4.1.
    PllOutput(PllClockSource, u8, u8),
}

impl MainClockSourceInfo {
    #[inline(always)]
    pub fn frequency(&self) -> Hertz {
        match self {
            MainClockSourceInfo::Irc => IRC_FREQ,
            MainClockSourceInfo::SysOsc(hertz) => hertz.clone(),
            MainClockSourceInfo::WwdtOsc(freq, div) => freq.frequency() / (*div as u32),
            MainClockSourceInfo::PllOutput(source, m, _) => source.frequency() * (*m as u32),
        }
    }
}

/// Provides options for the clock source to use in order to reach the specified frequency.
///
/// Most of the configuration for the clock rate is performed automatically. Note that you may not get the exact frequency you requested,
/// although an attempt will be made to get as close as possible.
#[derive(Clone, Debug)]
pub enum MainClockConfig {
    /// Use the IRC oscillator as the main clock.
    Irc,

    /// Use the system oscillator, running at the provided [Hertz], directly as the main clock.
    SysOsc(Hertz),

    /// Use the watchdog oscillator as the main clock.
    ///
    /// Note that although the watchdog oscillator uses the least power, it is also the least accurate (F_clkana can be +/- 40% of
    /// the listed frequency value).
    WwdtOsc,

    /// Use the System PLL, with the specified [PllClockSource], as the main clock.
    PllOutput(PllClockSource),
}

// By default, we use the IRC with a divider of 1, as this is available in all environments.
impl Default for MainClockConfig {
    #[inline(always)]
    fn default() -> Self {
        Self::Irc
    }
}

pub struct Clocks {
    pub source: MainClockSourceInfo,
    pub divider: u8,
    _private: (),
}

impl Clocks {
    #[inline(always)]
    pub unsafe fn new(source: MainClockSourceInfo, divider: u8) -> Self {
        Self {
            source,
            divider,
            _private: (),
        }
    }

    #[inline(always)]
    pub fn frequency(&self) -> Hertz {
        self.source.frequency() / (self.divider as u32)
    }
}

pub struct ClockConfig(());

pub enum ClockConfigError {
    TargetOutOfRange,

}

impl ClockConfig {
    pub unsafe fn new() -> Self {
        Self(())
    }

    
}

pub struct Syscon {
    pub clock_config: ClockConfig,
    _private: SYSCON,
}

impl Syscon {
    pub fn free(self) -> SYSCON {
        self._private
    }
}
