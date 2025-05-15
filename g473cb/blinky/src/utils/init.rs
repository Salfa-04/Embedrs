//!
//! # Init
//!
//! ## Reserved
//!
//! - PF0, PF1   for OSC
//! - PC14, PC15 for OSC32
//! - PA13, PA14 for SWD
//! - PB8        for BOOT0
//! - PG10       for RST
//!

use crate::hal::{Config, init, rcc, time::mhz};

pub fn sys_init() -> (embassy_stm32::Peripherals,) {
    defmt::debug!("System Initialization...");

    let peripherals = {
        let mut config = Config::default();
        let rcc = &mut config.rcc;
        let mux = &mut rcc.mux;

        rcc.boost = true;
        rcc.low_power_run = false;

        rcc.hsi = false; // HSI = 16MHz
        rcc.hsi48 = Some(rcc::Hsi48Config {
            sync_from_usb: false, // Enable CRS for USB
        });

        rcc.hse = Some(rcc::Hse {
            freq: mhz(25), // HSE = 25MHz
            mode: rcc::HseMode::Oscillator,
        });

        rcc.pll = Some(rcc::Pll {
            source: rcc::PllSource::HSE,    //  25MHz
            prediv: rcc::PllPreDiv::DIV5,   //   5MHz
            mul: rcc::PllMul::MUL64,        // 320MHz
            divr: Some(rcc::PllRDiv::DIV2), // 160MHz for SysClk
            divq: Some(rcc::PllQDiv::DIV2), // 160MHz for FDCAN
            divp: Some(rcc::PllPDiv::DIV2), // 160MHz for ADC
        });

        rcc.sys = rcc::Sysclk::PLL1_R;
        mux.adc12sel = rcc::mux::Adcsel::PLL1_P;
        mux.adc345sel = rcc::mux::Adcsel::PLL1_P;
        mux.fdcansel = rcc::mux::Fdcansel::PLL1_Q;

        rcc.ls = rcc::LsConfig::default_lsi(); // LSI = 32kHz
        rcc.ahb_pre = rcc::AHBPrescaler::DIV1; // 160MHz
        rcc.apb1_pre = rcc::APBPrescaler::DIV1; // 160MHz
        rcc.apb2_pre = rcc::APBPrescaler::DIV1; // 160MHz
        mux.clk48sel = rcc::mux::Clk48sel::HSI48; // HSI48MHz

        init(config) // SysClock = xMHz
    };

    (peripherals,)
}
