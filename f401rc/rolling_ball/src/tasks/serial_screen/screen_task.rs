use {super::IntRqst, crate::hal};

use blocking_mutex::raw::ThreadModeRawMutex as RM;
use defmt::{debug, error};
use embassy_sync::{blocking_mutex, mutex::Mutex, signal::Signal};
use hal::{gpio, mode::Async, peripherals, usart};
use peripherals::{DMA1_CH5, DMA1_CH6, PA2, PA3, USART2};
use usart::{Config, Uart, UartTx};

static TARGET: Signal<RM, u8> = Signal::new();

pub fn screen_target() -> Option<u8> {
    TARGET.try_take()
}

async fn rcv_parse(rcv: &[u8], tx_buf: &mut u8) {
    for &data in rcv {
        match data {
            // Confirm
            10 => {
                TARGET.signal(*tx_buf);
            }

            // Reset
            13 => {
                *tx_buf = 5;
                TARGET.signal(*tx_buf);
            }

            // Set Value
            0..=9 => {
                *tx_buf = data;
            }

            _ => error!("Unexpected Value: [{}] in {}", data, rcv),
        }
    }

    screen_write(&[*tx_buf]).await;
}

static SCREEN_TX: Mutex<RM, Option<UartTx<Async>>> = Mutex::new(None);

#[super::task]
pub async fn screen_task(p: (USART2, PA3, PA2, DMA1_CH5, DMA1_CH6)) -> ! {
    let mut config = Config::default();
    config.baudrate = 9600;
    config.rx_pull = gpio::Pull::None;

    let (tx, mut rx) = Uart::new(p.0, p.1, p.2, IntRqst, p.4, p.3, config)
        .inspect_err(|e| defmt::error!("Serial Screen: {:?}", e))
        .unwrap()
        .split();

    let _ = { SCREEN_TX.lock().await.replace(tx) };
    debug!("Screen Serial Initialized!");

    let buffer = &mut [0u8; 64];

    // rcvd position
    let mut tx_buf = 5u8;

    loop {
        match rx.read_until_idle(buffer).await {
            Ok(x) => rcv_parse(&buffer[..x], &mut tx_buf).await,
            Err(x) => error!("Serial Screen: {:?}", x),
        }
    }
}

/// data in &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, b',']
/// max len == 10
async fn screen_write(data: &[u8]) {
    let mut g_tx = SCREEN_TX.lock().await;
    let Some(tx) = g_tx.as_mut() else {
        error!("Screen Serial: SCREEN_TX is not set!");
        return;
    };

    tx.blocking_write(b"main.t0.txt=\"C: ").unwrap();
    for &data in data {
        if data == 0 {
            break;
        }
        tx.blocking_write(&[data + b'0']).unwrap();
    }
    tx.blocking_write(&[b'"', 0xff, 0xff, 0xff]).unwrap();
}
