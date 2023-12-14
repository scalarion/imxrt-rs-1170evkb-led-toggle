//! build.rs

use imxrt_rt::{Family, RuntimeBuilder};

fn main() {
    // The iMXRT1170EVK has 64 MiB of flash.
    RuntimeBuilder::from_flexspi(Family::Imxrt1170, 64 * 1024 * 1024)
        .build()
        .unwrap();
}
