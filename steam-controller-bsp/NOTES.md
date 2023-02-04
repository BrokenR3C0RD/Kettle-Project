# Pin Assignments
Taken from https://github.com/greggersaurus/OpenSteamController/blob/master/ReverseEngineering/Luna_maiboard_V000456-00_rev3.md

Split into sections so I know what's getting assigned to what in the end.

## Triggers

Digital reads/powering needs basic GPIO

Analog reads need ADC

- PIO1_1:  Trigger Analog Enable (active low)
- PIO0_1:  (ISP entry) RT Digital 

### Right
- PIO1_13: RT Digital
- PIO0_13: RT Analog (when PIO1_1 = LOW)

### Left
- PIO1_27: LT Digital
- PIO0_11: LT Analog (when PIO1_1 = LOW)

## Joystick

Click/power needs basic GPIO

Analog reads need ADC

- PIO0_19: Joy Power (active high)
- PIO1_0:  Joy Click
- PIO0_12: Joy X (when PIO0_19 = HIGH)
- PIO0_14: Joy Y (when PIO0_19 = HIGH)

## USB

Usage requires basic GPIO and USB driver

- PIO0_3: USB VBus
- PIO0_6: USB Connect

## Trackpad
Usage requires basic GPIO + pin interrupts, SSP0 and `pinnacle` driver

### Shared
- PIO0_8:  Trackpad MISO
- PIO0_9:  Trackpad MOSI
- PIO1_29: Trackpad SCLK

### Left
- PIO1_26: Left Trackpad Click
- PIO1_6:  Left Trackpad Chip Select
- PIO1_16: Left Trackpad Data Ready

### Right
- PIO1_21: Right Trackpad Click
- PIO1_15: Right Trackpad Chip Select
- PIO0_23: Right Trackpad Data Ready

## MPU
Usage requires basic GPIO + pin interrupts and I2C 

- PIO0_4:  MPU SCL
- PIO0_5:  MPU SDA
- PIO1_23: MPU INT (Data Ready?)

## Haptics
Usage requires basic GPIO + counter/timer with interrupts

- PIO1_7:  Haptics Enable (active low)
- PIO0_18: Left Haptic (when PIO1_7 = LOW?)
- PIO1_12: Right Haptic (when PIO1_7 = LOW?)

## nRF (Bluetooth)
### UART
Usage requires USART

- PIO1_17: nRF RX
- PIO1_18: nRF TX

### SWD
Usage requires basic GPIO and `swd-bitbang`

- PIO1_24: nRF SWDIO
- PIO1_8:  nRF SWCLK

### Other
- PIO1_5: nRF related?

## Buttons
Reading requires basic GPIO

- PIO0_17: A
- PIO1_22: B
- PIO1_9:  X
- PIO1_11: Y
- PIO1_20: Left Arrow
- PIO1_19: Guide/Steam Button
- PIO1_2:  Right Arrow
- PIO1_25: Left Grip
- PIO1_3:  Right Grip
- PIO1_4:  LB
- PIO1_14: RB


### Other
Requires counter/timers

- PIO0_21: Steam LED

## Unknown/unused
- PIO0_2:  related to BOD?
- PIO0_7:  unknown (driven low)
- PIO0_10: SWCLK (unused)
- PIO0_15: SWDIO (unused)
- PIO0_16: unused?
- PIO0_20: unused?
- PIO0_22: Voltage measuring?
- PIO1_10: unknown?
- PIO1_28: unknown? related to PIO0_7?