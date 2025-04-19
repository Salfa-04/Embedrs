use {super::IntRqst, crate::hal, crate::sync};

use crate::controller::consts::POSI;
use hal::{gpio, peripherals, usart};
use peripherals::{DMA2_CH2, PA10, USART1};
use sync::blocking_mutex::raw::ThreadModeRawMutex as RM;
use sync::{mutex::Mutex, signal::Signal};
use usart::{Config, UartRx};

static V_POSITION: Signal<RM, (f32, f32)> = Signal::new();
static V_POSITIONS: Mutex<RM, [(f32, f32); 10]> = Mutex::new(POSI);

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

pub async fn get_mv_positions() -> [(f32, f32); 10] {
    let rst = *V_POSITIONS.lock().await;
    defmt::debug!("Vision MV: {:?}", rst);
    rst
}

async fn vision_parse(data: &[u8]) {
    // {
    //     let s = core::str::from_utf8(data);
    //     defmt::debug!("Vision MV: {}", defmt::Debug2Format(&s));
    //     return;
    // }

    if data.contains(&b'{') && data.contains(&b'}') {
        defmt::debug!("Vision MV: {:?}", data);

        let Some(idx_f) = data.iter().position(|&x| x == b'{') else {
            unreachable!("Vision MV: No start bracket");
        };

        let Some(idx_b) = data.iter().position(|&x| x == b'}') else {
            unreachable!("Vision MV: No end bracket");
        };

        //  {(:),(:),..,(:)}
        // => (),(),(),..,(),
        let data = data[idx_f + 1..idx_b].trim_ascii();
        for (idx, data) in data.split(|&x| x == b',').enumerate() {
            if let Ok(x) = core::str::from_utf8(data.trim_ascii()) {
                if !x.starts_with('(') || !x.ends_with(')') || !x.contains(':') {
                    defmt::error!("Vision MV: {:?}", x);
                    continue;
                }

                if let Some((x, y)) = x[1..x.len() - 1].split_once(':') {
                    match (x.trim().parse::<f32>(), y.trim().parse::<f32>()) {
                        (Ok(x), Ok(y)) => {
                            let mut position = V_POSITIONS.lock().await;
                            if let Some(p) = position.get_mut(idx + 1) {
                                *p = (x, y);
                            }
                        }

                        _ => defmt::error!("Vision MV: [{:?}]", (x, y)),
                    }
                }
            }
        }
    } else {
        for data in data.split(|&x| x == b',') {
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

    let mut buffer = [0u8; 200];

    loop {
        match rx.read_until_idle(&mut buffer).await {
            Ok(x) => vision_parse(&buffer[..x]).await,
            Err(e) => defmt::error!("Vison MV: {:?}", e),
        }
    }
}
