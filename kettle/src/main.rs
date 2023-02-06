#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;
use lpc11uxx_hal as hal;

#[cortex_m_rt::entry]
fn entry() -> ! {
    defmt::println!("Hello, world!");

    let result = hal::rom_data::sidiv(10, 5);
    defmt::println!("Division result 1 (should miss cache): {}", result);

    let result = hal::rom_data::sidiv(6, 2);
    defmt::println!("Division result 2 (should hit cache): {}", result);

    for i in 1..5 {
        defmt::println!("Division results from intrinsic: {}", 10 / i);
    }

    let usbd = unsafe { &*(hal::rom_data::RomDriverTable::usb()) };
    let usbd_version = usbd.version;

    defmt::println!("USB driver version: {:#X}", usbd_version);

    let mut results = [0u32; 4];

    unsafe { hal::rom_data::iap_entry(&[54, 0, 0, 0, 0], &mut results) };

    defmt::println!("PartID = {:#X}", results[1]);

    loop {}
}