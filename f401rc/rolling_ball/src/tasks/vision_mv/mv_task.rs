use {super::IntRqst, crate::hal};

use hal::{gpio, peripherals, usart};
use peripherals::{DMA2_CH2, PA10, USART1};
use usart::{Config, UartRx};

#[super::task]
pub async fn mv_task(p: (USART1, PA10, DMA2_CH2)) -> ! {
    let mut config = Config::default();
    config.baudrate = 115200;
    config.rx_pull = gpio::Pull::Up;

    let mut rx = UartRx::new(p.0, IntRqst, p.1, p.2, config)
        .inspect_err(|e| defmt::error!("Vison MV: {:?}", e))
        .unwrap();

    let mut buffer = [0u8; 64];

    loop {
        match rx.read_until_idle(&mut buffer).await {
            Ok(x) => defmt::info!("Vison MV: {:?}", &buffer[..x]),
            Err(e) => defmt::error!("Vison MV: {:?}", e),
        }
    }
}
