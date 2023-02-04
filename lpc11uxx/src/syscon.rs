#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System memory remap"]
    pub sysmemremap: SYSMEMREMAP,
    #[doc = "0x04 - Peripheral reset control"]
    pub presetctrl: PRESETCTRL,
    #[doc = "0x08 - System PLL control"]
    pub syspllctrl: SYSPLLCTRL,
    #[doc = "0x0c - System PLL status"]
    pub syspllstat: SYSPLLSTAT,
    #[doc = "0x10 - USB PLL control"]
    pub usbpllctrl: USBPLLCTRL,
    #[doc = "0x14 - USB PLL status"]
    pub usbpllstat: USBPLLSTAT,
    _reserved6: [u8; 0x08],
    #[doc = "0x20 - System oscillator control"]
    pub sysoscctrl: SYSOSCCTRL,
    #[doc = "0x24 - Watchdog oscillator control"]
    pub wdtoscctrl: WDTOSCCTRL,
    _reserved8: [u8; 0x08],
    #[doc = "0x30 - System reset status register"]
    pub sysrststat: SYSRSTSTAT,
    _reserved9: [u8; 0x0c],
    #[doc = "0x40 - System PLL clock source select"]
    pub syspllclksel: SYSPLLCLKSEL,
    #[doc = "0x44 - System PLL clock source update enable"]
    pub syspllclkuen: SYSPLLCLKUEN,
    #[doc = "0x48 - USB PLL clock source select"]
    pub usbpllclksel: USBPLLCLKSEL,
    #[doc = "0x4c - USB PLL clock source update enable"]
    pub usbpllclkuen: USBPLLCLKUEN,
    _reserved13: [u8; 0x20],
    #[doc = "0x70 - Main clock source select"]
    pub mainclksel: MAINCLKSEL,
    #[doc = "0x74 - Main clock source update enable"]
    pub mainclkuen: MAINCLKUEN,
    #[doc = "0x78 - System clock divider"]
    pub sysahbclkdiv: SYSAHBCLKDIV,
    _reserved16: [u8; 0x04],
    #[doc = "0x80 - System clock control"]
    pub sysahbclkctrl: SYSAHBCLKCTRL,
    _reserved17: [u8; 0x10],
    #[doc = "0x94 - SSP0 clock divider"]
    pub ssp0clkdiv: SSP0CLKDIV,
    #[doc = "0x98 - UART clock divider"]
    pub uartclkdiv: UARTCLKDIV,
    #[doc = "0x9c - SSP1 clock divider"]
    pub ssp1clkdiv: SSP1CLKDIV,
    _reserved20: [u8; 0x20],
    #[doc = "0xc0 - USB clock source select"]
    pub usbclksel: USBCLKSEL,
    #[doc = "0xc4 - USB clock source update enable"]
    pub usbclkuen: USBCLKUEN,
    #[doc = "0xc8 - USB clock source divider"]
    pub usbclkdiv: USBCLKDIV,
    _reserved23: [u8; 0x14],
    #[doc = "0xe0 - CLKOUT clock source select"]
    pub clkoutsel: CLKOUTSEL,
    #[doc = "0xe4 - CLKOUT clock source update enable"]
    pub clkoutuen: CLKOUTUEN,
    #[doc = "0xe8 - CLKOUT clock divider"]
    pub clkoutdiv: CLKOUTDIV,
    _reserved26: [u8; 0x14],
    #[doc = "0x100 - POR captured PIO status 0"]
    pub pioporcap0: PIOPORCAP0,
    #[doc = "0x104 - POR captured PIO status 1"]
    pub pioporcap1: PIOPORCAP1,
    _reserved28: [u8; 0x48],
    #[doc = "0x150 - Brown-Out Detect"]
    pub bodctrl: BODCTRL,
    #[doc = "0x154 - System tick counter calibration"]
    pub systckcal: SYSTCKCAL,
    _reserved30: [u8; 0x18],
    #[doc = "0x170 - IQR delay. Allows trade-off between interrupt latency and determinism."]
    pub irqlatency: IRQLATENCY,
    #[doc = "0x174 - NMI Source Control"]
    pub nmisrc: NMISRC,
    #[doc = "0x178..0x198 - GPIO Pin Interrupt Select register 0"]
    pub pintsel: [PINTSEL; 8],
    #[doc = "0x198 - USB clock control"]
    pub usbclkctrl: USBCLKCTRL,
    #[doc = "0x19c - USB clock status"]
    pub usbclkst: USBCLKST,
    _reserved35: [u8; 0x64],
    #[doc = "0x204 - Start logic 0 interrupt wake-up enable register 0"]
    pub starterp0: STARTERP0,
    _reserved36: [u8; 0x0c],
    #[doc = "0x214 - Start logic 1 interrupt wake-up enable register 1"]
    pub starterp1: STARTERP1,
    _reserved37: [u8; 0x18],
    #[doc = "0x230 - Power-down states in deep-sleep mode"]
    pub pdsleepcfg: PDSLEEPCFG,
    #[doc = "0x234 - Power-down states for wake-up from deep-sleep"]
    pub pdawakecfg: PDAWAKECFG,
    #[doc = "0x238 - Power configuration register"]
    pub pdruncfg: PDRUNCFG,
    _reserved40: [u8; 0x01b8],
    #[doc = "0x3f4 - Device ID"]
    pub device_id: DEVICE_ID,
}
#[doc = "SYSMEMREMAP (rw) register accessor: an alias for `Reg<SYSMEMREMAP_SPEC>`"]
pub type SYSMEMREMAP = crate::Reg<sysmemremap::SYSMEMREMAP_SPEC>;
#[doc = "System memory remap"]
pub mod sysmemremap;
#[doc = "PRESETCTRL (rw) register accessor: an alias for `Reg<PRESETCTRL_SPEC>`"]
pub type PRESETCTRL = crate::Reg<presetctrl::PRESETCTRL_SPEC>;
#[doc = "Peripheral reset control"]
pub mod presetctrl;
#[doc = "SYSPLLCTRL (rw) register accessor: an alias for `Reg<SYSPLLCTRL_SPEC>`"]
pub type SYSPLLCTRL = crate::Reg<syspllctrl::SYSPLLCTRL_SPEC>;
#[doc = "System PLL control"]
pub mod syspllctrl;
#[doc = "SYSPLLSTAT (r) register accessor: an alias for `Reg<SYSPLLSTAT_SPEC>`"]
pub type SYSPLLSTAT = crate::Reg<syspllstat::SYSPLLSTAT_SPEC>;
#[doc = "System PLL status"]
pub mod syspllstat;
#[doc = "USBPLLCTRL (rw) register accessor: an alias for `Reg<USBPLLCTRL_SPEC>`"]
pub type USBPLLCTRL = crate::Reg<usbpllctrl::USBPLLCTRL_SPEC>;
#[doc = "USB PLL control"]
pub mod usbpllctrl;
#[doc = "USBPLLSTAT (r) register accessor: an alias for `Reg<USBPLLSTAT_SPEC>`"]
pub type USBPLLSTAT = crate::Reg<usbpllstat::USBPLLSTAT_SPEC>;
#[doc = "USB PLL status"]
pub mod usbpllstat;
#[doc = "SYSOSCCTRL (rw) register accessor: an alias for `Reg<SYSOSCCTRL_SPEC>`"]
pub type SYSOSCCTRL = crate::Reg<sysoscctrl::SYSOSCCTRL_SPEC>;
#[doc = "System oscillator control"]
pub mod sysoscctrl;
#[doc = "WDTOSCCTRL (rw) register accessor: an alias for `Reg<WDTOSCCTRL_SPEC>`"]
pub type WDTOSCCTRL = crate::Reg<wdtoscctrl::WDTOSCCTRL_SPEC>;
#[doc = "Watchdog oscillator control"]
pub mod wdtoscctrl;
#[doc = "SYSRSTSTAT (rw) register accessor: an alias for `Reg<SYSRSTSTAT_SPEC>`"]
pub type SYSRSTSTAT = crate::Reg<sysrststat::SYSRSTSTAT_SPEC>;
#[doc = "System reset status register"]
pub mod sysrststat;
#[doc = "SYSPLLCLKSEL (rw) register accessor: an alias for `Reg<SYSPLLCLKSEL_SPEC>`"]
pub type SYSPLLCLKSEL = crate::Reg<syspllclksel::SYSPLLCLKSEL_SPEC>;
#[doc = "System PLL clock source select"]
pub mod syspllclksel;
#[doc = "SYSPLLCLKUEN (rw) register accessor: an alias for `Reg<SYSPLLCLKUEN_SPEC>`"]
pub type SYSPLLCLKUEN = crate::Reg<syspllclkuen::SYSPLLCLKUEN_SPEC>;
#[doc = "System PLL clock source update enable"]
pub mod syspllclkuen;
#[doc = "USBPLLCLKSEL (rw) register accessor: an alias for `Reg<USBPLLCLKSEL_SPEC>`"]
pub type USBPLLCLKSEL = crate::Reg<usbpllclksel::USBPLLCLKSEL_SPEC>;
#[doc = "USB PLL clock source select"]
pub mod usbpllclksel;
#[doc = "USBPLLCLKUEN (rw) register accessor: an alias for `Reg<USBPLLCLKUEN_SPEC>`"]
pub type USBPLLCLKUEN = crate::Reg<usbpllclkuen::USBPLLCLKUEN_SPEC>;
#[doc = "USB PLL clock source update enable"]
pub mod usbpllclkuen;
#[doc = "MAINCLKSEL (rw) register accessor: an alias for `Reg<MAINCLKSEL_SPEC>`"]
pub type MAINCLKSEL = crate::Reg<mainclksel::MAINCLKSEL_SPEC>;
#[doc = "Main clock source select"]
pub mod mainclksel;
#[doc = "MAINCLKUEN (rw) register accessor: an alias for `Reg<MAINCLKUEN_SPEC>`"]
pub type MAINCLKUEN = crate::Reg<mainclkuen::MAINCLKUEN_SPEC>;
#[doc = "Main clock source update enable"]
pub mod mainclkuen;
#[doc = "SYSAHBCLKDIV (rw) register accessor: an alias for `Reg<SYSAHBCLKDIV_SPEC>`"]
pub type SYSAHBCLKDIV = crate::Reg<sysahbclkdiv::SYSAHBCLKDIV_SPEC>;
#[doc = "System clock divider"]
pub mod sysahbclkdiv;
#[doc = "SYSAHBCLKCTRL (rw) register accessor: an alias for `Reg<SYSAHBCLKCTRL_SPEC>`"]
pub type SYSAHBCLKCTRL = crate::Reg<sysahbclkctrl::SYSAHBCLKCTRL_SPEC>;
#[doc = "System clock control"]
pub mod sysahbclkctrl;
#[doc = "SSP0CLKDIV (rw) register accessor: an alias for `Reg<SSP0CLKDIV_SPEC>`"]
pub type SSP0CLKDIV = crate::Reg<ssp0clkdiv::SSP0CLKDIV_SPEC>;
#[doc = "SSP0 clock divider"]
pub mod ssp0clkdiv;
#[doc = "UARTCLKDIV (rw) register accessor: an alias for `Reg<UARTCLKDIV_SPEC>`"]
pub type UARTCLKDIV = crate::Reg<uartclkdiv::UARTCLKDIV_SPEC>;
#[doc = "UART clock divider"]
pub mod uartclkdiv;
#[doc = "SSP1CLKDIV (rw) register accessor: an alias for `Reg<SSP1CLKDIV_SPEC>`"]
pub type SSP1CLKDIV = crate::Reg<ssp1clkdiv::SSP1CLKDIV_SPEC>;
#[doc = "SSP1 clock divider"]
pub mod ssp1clkdiv;
#[doc = "USBCLKSEL (rw) register accessor: an alias for `Reg<USBCLKSEL_SPEC>`"]
pub type USBCLKSEL = crate::Reg<usbclksel::USBCLKSEL_SPEC>;
#[doc = "USB clock source select"]
pub mod usbclksel;
#[doc = "USBCLKUEN (rw) register accessor: an alias for `Reg<USBCLKUEN_SPEC>`"]
pub type USBCLKUEN = crate::Reg<usbclkuen::USBCLKUEN_SPEC>;
#[doc = "USB clock source update enable"]
pub mod usbclkuen;
#[doc = "USBCLKDIV (rw) register accessor: an alias for `Reg<USBCLKDIV_SPEC>`"]
pub type USBCLKDIV = crate::Reg<usbclkdiv::USBCLKDIV_SPEC>;
#[doc = "USB clock source divider"]
pub mod usbclkdiv;
#[doc = "CLKOUTSEL (rw) register accessor: an alias for `Reg<CLKOUTSEL_SPEC>`"]
pub type CLKOUTSEL = crate::Reg<clkoutsel::CLKOUTSEL_SPEC>;
#[doc = "CLKOUT clock source select"]
pub mod clkoutsel;
#[doc = "CLKOUTUEN (rw) register accessor: an alias for `Reg<CLKOUTUEN_SPEC>`"]
pub type CLKOUTUEN = crate::Reg<clkoutuen::CLKOUTUEN_SPEC>;
#[doc = "CLKOUT clock source update enable"]
pub mod clkoutuen;
#[doc = "CLKOUTDIV (rw) register accessor: an alias for `Reg<CLKOUTDIV_SPEC>`"]
pub type CLKOUTDIV = crate::Reg<clkoutdiv::CLKOUTDIV_SPEC>;
#[doc = "CLKOUT clock divider"]
pub mod clkoutdiv;
#[doc = "PIOPORCAP0 (r) register accessor: an alias for `Reg<PIOPORCAP0_SPEC>`"]
pub type PIOPORCAP0 = crate::Reg<pioporcap0::PIOPORCAP0_SPEC>;
#[doc = "POR captured PIO status 0"]
pub mod pioporcap0;
#[doc = "PIOPORCAP1 (r) register accessor: an alias for `Reg<PIOPORCAP1_SPEC>`"]
pub type PIOPORCAP1 = crate::Reg<pioporcap1::PIOPORCAP1_SPEC>;
#[doc = "POR captured PIO status 1"]
pub mod pioporcap1;
#[doc = "BODCTRL (rw) register accessor: an alias for `Reg<BODCTRL_SPEC>`"]
pub type BODCTRL = crate::Reg<bodctrl::BODCTRL_SPEC>;
#[doc = "Brown-Out Detect"]
pub mod bodctrl;
#[doc = "SYSTCKCAL (rw) register accessor: an alias for `Reg<SYSTCKCAL_SPEC>`"]
pub type SYSTCKCAL = crate::Reg<systckcal::SYSTCKCAL_SPEC>;
#[doc = "System tick counter calibration"]
pub mod systckcal;
#[doc = "IRQLATENCY (rw) register accessor: an alias for `Reg<IRQLATENCY_SPEC>`"]
pub type IRQLATENCY = crate::Reg<irqlatency::IRQLATENCY_SPEC>;
#[doc = "IQR delay. Allows trade-off between interrupt latency and determinism."]
pub mod irqlatency;
#[doc = "NMISRC (rw) register accessor: an alias for `Reg<NMISRC_SPEC>`"]
pub type NMISRC = crate::Reg<nmisrc::NMISRC_SPEC>;
#[doc = "NMI Source Control"]
pub mod nmisrc;
#[doc = "PINTSEL (rw) register accessor: an alias for `Reg<PINTSEL_SPEC>`"]
pub type PINTSEL = crate::Reg<pintsel::PINTSEL_SPEC>;
#[doc = "GPIO Pin Interrupt Select register 0"]
pub mod pintsel;
#[doc = "USBCLKCTRL (rw) register accessor: an alias for `Reg<USBCLKCTRL_SPEC>`"]
pub type USBCLKCTRL = crate::Reg<usbclkctrl::USBCLKCTRL_SPEC>;
#[doc = "USB clock control"]
pub mod usbclkctrl;
#[doc = "USBCLKST (r) register accessor: an alias for `Reg<USBCLKST_SPEC>`"]
pub type USBCLKST = crate::Reg<usbclkst::USBCLKST_SPEC>;
#[doc = "USB clock status"]
pub mod usbclkst;
#[doc = "STARTERP0 (rw) register accessor: an alias for `Reg<STARTERP0_SPEC>`"]
pub type STARTERP0 = crate::Reg<starterp0::STARTERP0_SPEC>;
#[doc = "Start logic 0 interrupt wake-up enable register 0"]
pub mod starterp0;
#[doc = "STARTERP1 (rw) register accessor: an alias for `Reg<STARTERP1_SPEC>`"]
pub type STARTERP1 = crate::Reg<starterp1::STARTERP1_SPEC>;
#[doc = "Start logic 1 interrupt wake-up enable register 1"]
pub mod starterp1;
#[doc = "PDSLEEPCFG (rw) register accessor: an alias for `Reg<PDSLEEPCFG_SPEC>`"]
pub type PDSLEEPCFG = crate::Reg<pdsleepcfg::PDSLEEPCFG_SPEC>;
#[doc = "Power-down states in deep-sleep mode"]
pub mod pdsleepcfg;
#[doc = "PDAWAKECFG (rw) register accessor: an alias for `Reg<PDAWAKECFG_SPEC>`"]
pub type PDAWAKECFG = crate::Reg<pdawakecfg::PDAWAKECFG_SPEC>;
#[doc = "Power-down states for wake-up from deep-sleep"]
pub mod pdawakecfg;
#[doc = "PDRUNCFG (rw) register accessor: an alias for `Reg<PDRUNCFG_SPEC>`"]
pub type PDRUNCFG = crate::Reg<pdruncfg::PDRUNCFG_SPEC>;
#[doc = "Power configuration register"]
pub mod pdruncfg;
#[doc = "DEVICE_ID (r) register accessor: an alias for `Reg<DEVICE_ID_SPEC>`"]
pub type DEVICE_ID = crate::Reg<device_id::DEVICE_ID_SPEC>;
#[doc = "Device ID"]
pub mod device_id;
