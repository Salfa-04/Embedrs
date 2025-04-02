//!
//! # Init
//!

use crate::hal::{Config, init, rcc, rcc::mux, time::mhz};

pub fn sys_init() -> (embassy_stm32::Peripherals,) {
    let peripherals = {
        let mut config = Config::default();
        let rcc = &mut config.rcc;
        let mux = &mut rcc.mux;

        {
            rcc.csi = false; // NoUsed 4MHz
            rcc.hsi = Some(rcc::HSIPrescaler::DIV1); // 64MHz
            rcc.hsi48 = Some(rcc::Hsi48Config::default()); // 48MHz
            rcc.hse = Some(rcc::Hse {
                freq: mhz(24), // 24MHz
                mode: rcc::HseMode::Oscillator,
            });

            mux.cecsel = mux::Cecsel::LSI; // 32kHz
            mux.rngsel = mux::Rngsel::HSI48; // 48MHz
            mux.persel = mux::Persel::HSI; // 64MHz
        }

        {
            rcc.pll1 = Some(rcc::Pll {
                source: rcc::PllSource::HSE,   //  24MHz
                prediv: rcc::PllPreDiv::DIV3,  //   8MHz
                mul: rcc::PllMul::MUL65,       // 520MHz
                divp: Some(rcc::PllDiv::DIV1), // 520MHz
                divq: Some(rcc::PllDiv::DIV4), // 130MHz
                divr: None,                    // NoUsed
            });

            rcc.sys = rcc::Sysclk::PLL1_P; // 520MHz
            mux.fdcansel = mux::Fdcansel::PLL1_Q; // 130MHz

            rcc.d1c_pre = rcc::AHBPrescaler::DIV1; //  520MHz
            rcc.ahb_pre = rcc::AHBPrescaler::DIV2; //  260MHz
            rcc.apb1_pre = rcc::APBPrescaler::DIV2; // 130MHz
            rcc.apb2_pre = rcc::APBPrescaler::DIV2; // 130MHz
            rcc.apb3_pre = rcc::APBPrescaler::DIV2; // 130MHz
            rcc.apb4_pre = rcc::APBPrescaler::DIV2; // 130MHz

            rcc.timer_prescaler = rcc::TimerPrescaler::DefaultX4;
            rcc.voltage_scale = rcc::VoltageScale::Scale0;
            rcc.ls = rcc::LsConfig::default_lsi(); // 32kHz
            rcc.supply_config = rcc::SupplyConfig::LDO;

            mux.fmcsel = mux::Fmcsel::HCLK3; // 260MHz
            mux.octospisel = mux::Fmcsel::HCLK3; // 260MHz
            mux.lptim1sel = mux::Lptim1sel::PCLK1; // 130MHz
            mux.lptim2sel = mux::Lptim2sel::PCLK4; // 130MHz
            mux.lpuart1sel = mux::Lpuartsel::PCLK4; // 130MHz
            mux.usart16910sel = mux::Usart16910sel::PCLK2; // 130MHz
            mux.usart234578sel = mux::Usart234578sel::PCLK1; // 130MHz
            mux.i2c1235sel = mux::I2c1235sel::PCLK1; // 130MHz
            mux.i2c4sel = mux::I2c4sel::PCLK4; // 130MHz
        }

        {
            rcc.pll2 = Some(rcc::Pll {
                source: rcc::PllSource::HSE,   //  24MHz
                prediv: rcc::PllPreDiv::DIV12, //   2MHz
                mul: rcc::PllMul::MUL125,      // 250MHz
                divp: Some(rcc::PllDiv::DIV1), // 250MHz
                divq: Some(rcc::PllDiv::DIV1), // 250MHz
                divr: Some(rcc::PllDiv::DIV1), // 250MHz
            });

            mux.spi123sel = mux::Saisel::PLL2_P; // 250MHz
            mux.spi45sel = mux::Spi45sel::PLL2_Q; // 250MHz
            mux.spi6sel = mux::Spi6sel::PLL2_Q; // 250MHz
            mux.sdmmcsel = mux::Sdmmcsel::PLL2_R; // 250MHz
            mux.spdifrxsel = mux::Spdifrxsel::PLL2_R; // 250MHz
        }

        {
            rcc.pll3 = Some(rcc::Pll {
                source: rcc::PllSource::HSE,   //  24MHz
                prediv: rcc::PllPreDiv::DIV8,  //   3MHz
                mul: rcc::PllMul::MUL208,      // 624MHz
                divp: Some(rcc::PllDiv::DIV4), // 156MHz
                divq: Some(rcc::PllDiv::DIV5), // 124MHz
                divr: Some(rcc::PllDiv::DIV4), // 156MHz
            });

            mux.sai1sel = mux::Saisel::PLL3_P; // 156MHz
            mux.usbsel = mux::Usbsel::PLL3_Q; // 124MHz
            mux.adcsel = mux::Adcsel::PLL3_R; // 156MHz}

            config.enable_debug_during_sleep = true; //
        }

        init(config) // SysClock = 550MHz
    };

    (peripherals,)
}
