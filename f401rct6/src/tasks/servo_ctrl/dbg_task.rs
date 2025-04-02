//!
//! # Dbg Task
//!

use crate::hal::{peripherals, usart};
use defmt::{error, info};
use peripherals::{DMA2_CH2, PA10, USART1};
use usart::{Config, UartRx};

#[super::task]
pub async fn dbg_task(p: (USART1, PA10, DMA2_CH2)) {
    let mut config = Config::default();
    config.baudrate = 115200;

    let mut rx = UartRx::new(p.0, super::IntRqst, p.1, p.2, config)
        .inspect_err(|e| error!("DBG UART1 Init Error: {:?}", e))
        .unwrap();

    info!("DBG UART1 Initialized!");

    let mut buffer = [0u8; 128];

    loop {
        match rx.read_until_idle(&mut buffer).await {
            Ok(x) => {
                super::pwm_task::pwm_set_dbg(&buffer[..x]).await;
            }

            Err(e) => error!("DBG Read Error: {:?}", e),
        }
    }
}
