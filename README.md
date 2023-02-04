Kettle Project
===

A reimplementation of the Steam Controller firmware in Rust.

Subprojects
---

- `[lpc11uxx]` - A Peripheral Access Crate for the NXP LPC11Uxx series of Cortex-M0 MCUs
- `[lpc11uxx-hal]` - A Hardware Abstraction Layer for the NXP LPC11Uxx series of Cortex-M0 MCUs
- `[pinnacle]` - An embedded-hal driver for the Cirque Pinnacle Touch Controller
- `[mpu6500]` - An embedded-hal driver for the Invensense MPU-6500 6-axis MotionTracking controller
- `[swd-bitbang]` - A embedded-hal host-side driver for the ARM Serial Wire Debug interface, implemented via GPIO bitbang
- `[steam-controller-bsp]` - A Board Support Package for the Steam Controller board.
- `[kettle]` - A custom reimplementation of the Steam Controller firmware

Goals
---

The primary goal is to get our own code running on both the NXP LPC11U37F/501 and the Nordic nRF51822.
Getting code running on the LPC (master) is the easy part. The nRF chip (radio), on the other hand, is where things get sticky.
Master communicates with Radio over UART, for all except updates. During a system update, master talks to Radio via SWD.
To get our own code running on the nRF, we will need to bitbang SWD over the GPIO pins, and then flash the nRF with some firmware to give us
control over the chip.

The bottleneck on this is that the LPC11U37 does not have any Rust support, aside from a PAC uploaded by the lpc-pac team. I've taken the liberty to modify the SVD to build with the latest svd2rust, and to fix some bad naming on enumeratedValues, but to get any further,
there needs to be a functioning HAL for this MCU.

In order to communicate with the trackpads, we need support for the SPI peripheral. The 6-axis motion sensor, we need I2C. For haptics and lighting, we need the counter/timers. For communication with Radio, we need USART. And for all of those, we need GPIO support. So a full HAL implementation is needed.

Why?
---
I haven't gotten a chance to really dig deep with embedded programming, so I decided to go all in on doing a ridiculous project first. No better way to learn than to do, right?

Also, I'm using this project to get reallyyyyyy comfortable with Rust. I've made a few attempts on this project so far, and I've learned more about how to structure my code in this crazy language as I've went along.

What's working?
---
Right now? Absolutely nothing.