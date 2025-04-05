use {super::IntRqst, crate::hal};

use embassy_sync::{self as sync, mutex::Mutex};
use hal::{gpio, peripherals, usart};
use peripherals::{DMA2_CH2, PA10, USART1};
use sync::blocking_mutex::raw::ThreadModeRawMutex as RM;
use usart::{Config, UartRx};

static V_POSITION: Mutex<RM, (f32, f32)> = Mutex::new((0.0, 0.0));

pub async fn get_mv_position() -> (f32, f32) {
    *V_POSITION.lock().await
}

async fn vision_parse(data: &[u8]) {
    let data = data.split(|&x| x == b',');

    for data in data {
        if let Ok(x) = core::str::from_utf8(data.trim_ascii()) {
            if !x.starts_with('[') || !x.ends_with(']') || !x.contains(':') {
                continue;
            }

            if let Some((x, y)) = x[1..x.len() - 1].split_once(':') {
                match (x.trim().parse::<f32>(), y.trim().parse::<f32>()) {
                    (Ok(x), Ok(y)) => {
                        *V_POSITION.lock().await = (x, y);
                    }

                    _ => defmt::error!("Vision MV: [{:?}]", (x, y)),
                }
            }
        }
    }
}

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
            Ok(x) => vision_parse(&buffer[..x]).await,
            Err(e) => defmt::error!("Vison MV: {:?}", e),
        }
    }
}
