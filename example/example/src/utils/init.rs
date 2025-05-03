//!
//! # Init
//!

use crate::hal::{Config, init};

pub fn sys_init() -> (embassy_stm32::Peripherals,) {
    defmt::debug!("System Initialization...");

    let peripherals = {
        let mut config = Config::default();
        let _rcc = &mut config.rcc;

        init(config) // SysClock = xMHz
    };

    (peripherals,)
}
