use super::IntRqst;
use crate::{hal, init_ticker};

use hal::{gpio, peripherals, usart};
use peripherals::{DMA1_CH5, DMA1_CH6, PA2, PA3, USART2};
use usart::{Config, Uart};

#[super::task]
pub async fn screen_task(p: (USART2, PA3, PA2, DMA1_CH5, DMA1_CH6)) -> ! {
    let mut t = init_ticker!(100);

    let mut config = Config::default();
    config.baudrate = 115200;
    config.rx_pull = gpio::Pull::Up;

    let mut _u = Uart::new(p.0, p.1, p.2, IntRqst, p.4, p.3, config)
        .inspect_err(|e| defmt::error!("Serial Screen: {:?}", e))
        .unwrap();
    let mut _buffer = [0u8; 64];

    loop {
        // if let Some(data) = screen.read().await {
        //     info!("Serial Screen: {:?}", data);
        // }

        t.next().await;
    }
}
