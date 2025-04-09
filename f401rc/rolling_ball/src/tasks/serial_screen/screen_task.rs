use {super::IntRqst, crate::hal};

use blocking_mutex::raw::ThreadModeRawMutex as RM;
use core::sync::atomic::{AtomicU8, Ordering::Relaxed};
use defmt::{debug, error, info};
use embassy_sync::{blocking_mutex, mutex::Mutex, signal::Signal};
use hal::{gpio, mode::Async, peripherals, usart};
use peripherals::{DMA1_CH5, DMA1_CH6, PA2, PA3, USART2};
use usart::{Config, Uart, UartTx};

static TARGET: Signal<RM, [u8; 10]> = Signal::new();

pub fn get_screen_fb() -> impl Future<Output = [u8; 10]> {
    TARGET.wait()
}

async fn rcv_parse(rcv: &[u8], tx_buf: &mut [u8; 10]) {
    /// STATES: u8
    /// 0: init
    /// 1: start
    static STATES: AtomicU8 = AtomicU8::new(0);

    for &data in rcv {
        match data {
            16 => {
                tx_buf.fill(0);
                STATES.store(0, Relaxed);
            }

            19 => {
                STATES.store(1, Relaxed);
                TARGET.signal(*tx_buf);
            }

            0..=9 => {
                if STATES.load(Relaxed) == 0 {
                    for tx_data in tx_buf.iter_mut() {
                        if *tx_data == 0 {
                            *tx_data = data;
                            break;
                        }
                    }
                }
            }

            _ => info!("Unexpected Value: [{}] in {}", data, rcv),
        }
    }
}

static SCREEN_TX: Mutex<RM, Option<UartTx<Async>>> = Mutex::new(None);

#[super::task]
pub async fn screen_task(p: (USART2, PA3, PA2, DMA1_CH5, DMA1_CH6)) -> ! {
    let mut config = Config::default();
    config.baudrate = 9600;
    config.rx_pull = gpio::Pull::None;

    let (tx, rx) = Uart::new(p.0, p.1, p.2, IntRqst, p.4, p.3, config)
        .inspect_err(|e| defmt::error!("Serial Screen: {:?}", e))
        .unwrap()
        .split();

    let _ = SCREEN_TX.lock().await.replace(tx);
    debug!("Screen Serial Initialized!");

    let (buffer, dma_buf) = (&mut [0u8; 64], &mut [0u8; 64]);
    let mut rx = rx.into_ring_buffered(dma_buf);
    let mut tx_buf = [0u8; 10];

    loop {
        match rx.read(buffer).await {
            Ok(x) => {
                rcv_parse(&buffer[..x], &mut tx_buf).await;
            }

            Err(x) => error!("Serial Screen: {:?}", x),
        }

        screen_write(tx_buf.as_slice()).await;
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

    tx.blocking_write(b"main.t0.txt=\"").unwrap();
    for &data in data {
        if data == 0 {
            break;
        }
        tx.blocking_write(&[data + b'0']).unwrap();
    }
    tx.blocking_write(&[b'"', 0xff, 0xff, 0xff]).unwrap();
}
