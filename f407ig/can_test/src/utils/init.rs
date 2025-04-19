//!
//! # Init
//!

use crate::hal::{Config, init, rcc, time::mhz};

pub fn sys_init() -> (embassy_stm32::Peripherals,) {
    defmt::debug!("System Initialization...");

    let peripherals = {
        let mut config = Config::default();
        let rcc = &mut config.rcc;

        rcc.hsi = false;
        rcc.hse = Some(rcc::Hse {
            freq: mhz(12),
            mode: rcc::HseMode::Oscillator,
        });

        rcc.pll_src = rcc::PllSource::HSE; // 12MHz

        rcc.pll = Some(rcc::Pll {
            prediv: rcc::PllPreDiv::DIV6,   //   2MHz
            mul: rcc::PllMul::MUL168,       // 168MHz
            divp: Some(rcc::PllPDiv::DIV2), //  84MHz
            divq: Some(rcc::PllQDiv::DIV7), //  48MHz
            divr: None,
        });

        rcc.plli2s = Some(rcc::Pll {
            prediv: rcc::PllPreDiv::DIV6, //   2MHz
            mul: rcc::PllMul::MUL192,     // 384MHz
            divp: None,
            divq: None,
            divr: Some(rcc::PllRDiv::DIV2), // 192MHz
        });

        rcc.sys = rcc::Sysclk::PLL1_P; // 168MHz
        rcc.ahb_pre = rcc::AHBPrescaler::DIV1; // 168MHz
        rcc.apb1_pre = rcc::APBPrescaler::DIV4; //  42MHz
        rcc.apb2_pre = rcc::APBPrescaler::DIV2; //  84MHz

        rcc.ls = rcc::LsConfig::default_lsi(); // LSI = 32kHz
        rcc.mux.clk48sel = rcc::mux::Clk48sel::PLL1_Q; // 48MHz
        rcc.mux.sdiosel = rcc::mux::Sdiosel::SYS; // 168MHz

        init(config) // SysClock = 168MHz
    };

    (peripherals,)
}
