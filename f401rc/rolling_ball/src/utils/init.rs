//!
//! # Init
//!

use crate::hal::{Config, init, rcc, time::mhz};

pub fn sys_init() -> (embassy_stm32::Peripherals,) {
    let peripherals = {
        let mut config = Config::default();
        let rcc = &mut config.rcc;

        rcc.hsi = false;
        rcc.hse = Some(rcc::Hse {
            freq: mhz(8),
            mode: rcc::HseMode::Oscillator,
        });

        rcc.pll_src = rcc::PllSource::HSE; // 8MHz

        rcc.pll = Some(rcc::Pll {
            prediv: rcc::PllPreDiv::DIV4,   // M:   2MHz
            mul: rcc::PllMul::MUL168,       // N: 336MHz
            divp: Some(rcc::PllPDiv::DIV4), // P:  84MHz
            divq: Some(rcc::PllQDiv::DIV7), // Q:  48MHz
            divr: None,                     // R:   None
        });

        rcc.plli2s = Some(rcc::Pll {
            prediv: rcc::PllPreDiv::DIV4,   // M:   2MHz
            mul: rcc::PllMul::MUL192,       // N: 384MHz
            divp: None,                     // P:   None
            divq: None,                     // Q:   None
            divr: Some(rcc::PllRDiv::DIV2), // R: 192MHz
        });

        rcc.sys = rcc::Sysclk::PLL1_P;
        rcc.ahb_pre = rcc::AHBPrescaler::DIV1;
        rcc.apb1_pre = rcc::APBPrescaler::DIV2;
        rcc.apb2_pre = rcc::APBPrescaler::DIV1;

        rcc.ls = rcc::LsConfig::default_lsi();
        rcc.mux.clk48sel = rcc::mux::Clk48sel::PLL1_Q;
        rcc.mux.sdiosel = rcc::mux::Sdiosel::SYS; // 84Mhz

        init(config) // SysClock = 84Mhz
    };

    (peripherals,)
}
