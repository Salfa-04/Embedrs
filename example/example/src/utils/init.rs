//!
//! # Init
//!
//! ## Reserved
//!
//!  (for stm32g473cbt9)
//! - PF0, PF1   for OSC
//! - PC14, PC15 for OSC32
//! - PA13, PA14 for SWD
//! - PB8        for BOOT0
//! - PG10       for RST
//!

use crate::hal::{init, Config};

pub fn sys_init() -> (embassy_stm32::Peripherals,) {
    defmt::debug!("System Initialization...");

    let peripherals = {
        let mut config = Config::default();
        let _rcc = &mut config.rcc;

        init(config) // SysClock = xMHz
    };

    (peripherals,)
}
