#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog mode register. This register contains the basic mode and status of the Watchdog Timer."]
    pub mod_: MOD,
    #[doc = "0x04 - Watchdog timer constant register. This 24-bit register determines the time-out value."]
    pub tc: TC,
    #[doc = "0x08 - Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in WDTC."]
    pub feed: FEED,
    #[doc = "0x0c - Watchdog timer value register. This 24-bit register reads out the current value of the Watchdog timer."]
    pub tv: TV,
    #[doc = "0x10 - Watchdog clock select register."]
    pub clksel: CLKSEL,
    #[doc = "0x14 - Watchdog Warning Interrupt compare value."]
    pub warnint: WARNINT,
    #[doc = "0x18 - Watchdog Window compare value."]
    pub window: WINDOW,
}
#[doc = "MOD (rw) register accessor: an alias for `Reg<MOD_SPEC>`"]
pub type MOD = crate::Reg<mod_::MOD_SPEC>;
#[doc = "Watchdog mode register. This register contains the basic mode and status of the Watchdog Timer."]
pub mod mod_;
#[doc = "TC (rw) register accessor: an alias for `Reg<TC_SPEC>`"]
pub type TC = crate::Reg<tc::TC_SPEC>;
#[doc = "Watchdog timer constant register. This 24-bit register determines the time-out value."]
pub mod tc;
#[doc = "FEED (w) register accessor: an alias for `Reg<FEED_SPEC>`"]
pub type FEED = crate::Reg<feed::FEED_SPEC>;
#[doc = "Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in WDTC."]
pub mod feed;
#[doc = "TV (r) register accessor: an alias for `Reg<TV_SPEC>`"]
pub type TV = crate::Reg<tv::TV_SPEC>;
#[doc = "Watchdog timer value register. This 24-bit register reads out the current value of the Watchdog timer."]
pub mod tv;
#[doc = "CLKSEL (rw) register accessor: an alias for `Reg<CLKSEL_SPEC>`"]
pub type CLKSEL = crate::Reg<clksel::CLKSEL_SPEC>;
#[doc = "Watchdog clock select register."]
pub mod clksel;
#[doc = "WARNINT (rw) register accessor: an alias for `Reg<WARNINT_SPEC>`"]
pub type WARNINT = crate::Reg<warnint::WARNINT_SPEC>;
#[doc = "Watchdog Warning Interrupt compare value."]
pub mod warnint;
#[doc = "WINDOW (rw) register accessor: an alias for `Reg<WINDOW_SPEC>`"]
pub type WINDOW = crate::Reg<window::WINDOW_SPEC>;
#[doc = "Watchdog Window compare value."]
pub mod window;
