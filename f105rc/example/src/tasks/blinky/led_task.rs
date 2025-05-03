//!
//! # LED Task
//!

use crate::{hal, init_ticker};
use hal::{peripherals, spi, time};

use peripherals::{DMA1_CH3, PA7, SPI1};
use spi::{BitOrder, Config, Polarity, Spi};
use time::mhz;

type Signal = u8;

const WS2812_GRB_SIZE: usize = 24; // 8 bits for each of the 3 colors
const N0: Signal = 0b11000000u8; // WS2812 Input 0
const N1: Signal = 0b11111100u8; // WS2812 Input 1

const RGB_CHANGE_HZ: u32 = 1000;
const RGB_FLOW_COLOR: [u32; 7] = [
    0xFF0000FF, // blue
    0x0000FF00, // green(dark)
    0xFFFF0000, // red
    0x000000FF, // blue(dark)
    0xFF00FF00, // green
    0x00FF0000, // red(dark)
    0xFF0000FF, // blue
];

#[embassy_executor::task]
pub async fn led_task(p: (SPI1, PA7, DMA1_CH3)) -> ! {
    let mut t = init_ticker!(1000 / RGB_CHANGE_HZ as u64);

    let mut config = Config::default();
    config.frequency = mhz(9);
    config.mode.polarity = Polarity::IdleLow;
    config.bit_order = BitOrder::MsbFirst;

    let mut ws2812 = Spi::new_txonly_nosck(p.0, p.1, p.2, config);

    loop {
        for i in 0..RGB_FLOW_COLOR.len() - 1 {
            let mut alpha = ((RGB_FLOW_COLOR[i] & 0xFF000000) >> 24) as f32;
            let mut red = ((RGB_FLOW_COLOR[i] & 0x00FF0000) >> 16) as f32;
            let mut green = ((RGB_FLOW_COLOR[i] & 0x0000FF00) >> 8) as f32;
            let mut blue = ((RGB_FLOW_COLOR[i] & 0x000000FF) >> 0) as f32;

            let mut delta_alpha = ((RGB_FLOW_COLOR[i + 1] & 0xFF000000) >> 24) as f32 - alpha;
            let mut delta_red = ((RGB_FLOW_COLOR[i + 1] & 0x00FF0000) >> 16) as f32 - red;
            let mut delta_green = ((RGB_FLOW_COLOR[i + 1] & 0x0000FF00) >> 8) as f32 - green;
            let mut delta_blue = ((RGB_FLOW_COLOR[i + 1] & 0x000000FF) >> 0) as f32 - blue;

            delta_alpha /= RGB_CHANGE_HZ as f32;
            delta_red /= RGB_CHANGE_HZ as f32;
            delta_green /= RGB_CHANGE_HZ as f32;
            delta_blue /= RGB_CHANGE_HZ as f32;

            for _ in 0..RGB_CHANGE_HZ {
                alpha += delta_alpha;
                red += delta_red;
                green += delta_green;
                blue += delta_blue;

                let grb: [(Signal, usize); 3] = [
                    (((green * alpha) * u8::MAX as f32 / u16::MAX as f32) as _, 0),
                    (((red * alpha) * u8::MAX as f32 / u16::MAX as f32) as _, 8),
                    (((blue * alpha) * u8::MAX as f32 / u16::MAX as f32) as _, 16),
                ];

                let mut buffer = [0 as Signal; WS2812_GRB_SIZE];

                grb.into_iter()
                    .flat_map(|(color, start)| {
                        (0..8).map(move |i| (start + i, (color >> (7 - i)) & 1))
                    })
                    .for_each(|(idx, bit)| buffer[idx] = if bit != 0 { N1 } else { N0 });

                defmt::info!("WS2812: {:?}", ws2812.write(&buffer).await);

                t.next().await;
            }
        }
    }
}
