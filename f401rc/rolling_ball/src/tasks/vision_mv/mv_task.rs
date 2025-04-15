use {super::IntRqst, crate::hal};

use embassy_sync::{self as sync, signal::Signal};
use hal::{gpio, peripherals, usart};
use peripherals::{DMA2_CH2, PA10, USART1};
use sync::blocking_mutex::raw::ThreadModeRawMutex as RM;
use usart::{Config, UartRx};

static V_POSITION: Signal<RM, (f32, f32)> = Signal::new();

pub fn get_mv_position() -> Option<(u16, u16)> {
    if V_POSITION.signaled() {
        if let Some((x, y)) = V_POSITION.try_take() {
            assert!(x >= 0.0 && x <= u16::MAX as _);
            assert!(y >= 0.0 && y <= u16::MAX as _);

            Some((x as _, y as _))
        } else {
            None
        }
    } else {
        None
    }
}

fn vision_parse(data: &[u8]) {
    // {
    // let s = core::str::from_utf8(data);
    // defmt::debug!("Vision MV: {}", defmt::Debug2Format(&s));
    //     return;
    // }

    let data = data.split(|&x| x == b',');

    for data in data {
        if let Ok(x) = core::str::from_utf8(data.trim_ascii()) {
            if !x.starts_with('[') || !x.ends_with(']') || !x.contains(':') {
                continue;
            }

            if let Some((x, y)) = x[1..x.len() - 1].split_once(':') {
                match (x.trim().parse::<f32>(), y.trim().parse::<f32>()) {
                    (Ok(x), Ok(y)) => {
                        V_POSITION.signal((x, y));
                        // defmt::debug!("Vision MV: [{:?}]", (x, y));
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
    config.baudrate = 500_000;
    config.rx_pull = gpio::Pull::Up;

    let mut rx = UartRx::new(p.0, IntRqst, p.1, p.2, config)
        .inspect_err(|e| defmt::error!("Vison MV: {:?}", e))
        .unwrap();
    defmt::debug!("Vision MV Initialized!");

    let mut buffer = [0u8; 64];

    loop {
        match rx.read_until_idle(&mut buffer).await {
            Ok(x) => vision_parse(&buffer[..x]),
            Err(e) => defmt::error!("Vison MV: {:?}", e),
        }
    }
}
