//! main.rs

#![no_main]
#![no_std]

// Include the boot header like this. Otherwise,
// it may be removed by the linker.
use imxrt1170evk_fcb as _;
use panic_halt as _;

// The entry macro adorns your main function.
use imxrt_rt::entry;

use cortex_m as cm7;
use imxrt_hal as hal;
use imxrt_ral as ral;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let iomuxc = unsafe { ral::iomuxc::IOMUXC::instance() };
    let iomuxc_lpsr = unsafe { ral::iomuxc_lpsr::IOMUXC_LPSR::instance() };
    let gpio9 = unsafe { ral::gpio::GPIO9::instance() };

    let mut gpio9 = hal::gpio::Port::new(gpio9);
    let pads = hal::iomuxc::into_pads(iomuxc, iomuxc_lpsr);

    let led1 = gpio9.output(pads.gpio_ad.p04);
    let led2 = gpio9.output(pads.gpio_ad.p26);

    led1.set();
    led2.clear();

    let mut counter: u32 = 0;

    loop {
        counter += 1;
        rprintln!("iteration: {:04}", counter);
        led1.toggle();
        led2.toggle();
        cm7::asm::delay(200_000_000);
    }
}
