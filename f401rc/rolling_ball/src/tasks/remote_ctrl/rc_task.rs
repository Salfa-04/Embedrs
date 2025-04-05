//!
//! # UART Task
//!

use super::sbus::SBusPacket as Packet;
use super::sbus::SBusPacketParser as Parser;
use crate::hal::{peripherals, usart};

use defmt::{Format, debug, error};
use embassy_sync::{blocking_mutex::raw, mutex::Mutex};
use raw::ThreadModeRawMutex as RM;
use usart::{Config, DataBits, Parity, StopBits, UartRx};

use peripherals::{DMA2_CH1 as DMA_RX, PA12 as UART_RX, USART6 as UART_PERI};
static RC_DATA: Mutex<RM, DjiSBusPacket> = Mutex::new(DjiSBusPacket::new());

pub async fn get_rc_data() -> DjiSBusPacket {
    RC_DATA.lock().await.clone()
}

#[derive(Debug, Format, Clone)]
pub struct DjiSBusPacket {
    pub ch_l_hori: i16,
    pub ch_l_vert: i16,
    pub ch_r_hori: i16,
    pub ch_r_vert: i16,
    pub sw_left: i8,
    pub sw_right: i8,
}

#[super::task]
pub async fn rc_task(p: (UART_PERI, UART_RX, DMA_RX)) -> ! {
    let mut config = Config::default();
    config.baudrate = 100000;
    config.data_bits = DataBits::DataBits8;
    config.parity = Parity::ParityEven;
    config.stop_bits = StopBits::STOP2;

    let mut rx = UartRx::new(p.0, super::IntRqst, p.1, p.2, config)
        .inspect_err(|e| error!("UART Init Error: {:?}", e))
        .unwrap();

    debug!("Remote Controller Initialized!");

    let mut buffer = [0u8; 48];
    let mut parser = Parser::new();

    loop {
        match rx.read_until_idle(&mut buffer).await {
            Ok(x) => {
                parser.push_bytes(&buffer[..x]);
                if let Some(x) = parser.try_parse() {
                    *RC_DATA.lock().await = x.into();
                    // debug!("RC Data: {:?}", DjiSBusPacket::from(x));
                }
            }

            Err(e) => error!("RC Read Error: {:?}", e),
        };
    }
}

impl DjiSBusPacket {
    const DJI_MIN: u16 = 364;
    const DJI_MAX: u16 = 1684;

    const DJI_RANGE: i16 = (Self::DJI_MAX - Self::DJI_MIN) as i16;
    const DJI_MIDDL: i16 = (Self::DJI_MAX + Self::DJI_MIN) as i16 / 2;

    const fn new() -> DjiSBusPacket {
        unsafe { core::mem::zeroed() }
    }
}

impl From<Packet> for DjiSBusPacket {
    fn from(value: Packet) -> Self {
        let switch = |v: i16| -> i8 {
            match v {
                x if x < 0 => -1,
                x if x > 0 => 1,
                _ => 0,
            }
        };

        let s = Self {
            ch_l_hori: value.channels[3] as i16 - Self::DJI_MIDDL,
            ch_l_vert: value.channels[2] as i16 - Self::DJI_MIDDL,
            ch_r_hori: value.channels[0] as i16 - Self::DJI_MIDDL,
            ch_r_vert: value.channels[1] as i16 - Self::DJI_MIDDL,
            sw_left: switch(value.channels[5] as i16 - Self::DJI_MIDDL),
            sw_right: switch(value.channels[6] as i16 - Self::DJI_MIDDL),
        };

        assert!(s.ch_l_hori >= -Self::DJI_RANGE && s.ch_l_hori <= Self::DJI_RANGE);
        assert!(s.ch_l_vert >= -Self::DJI_RANGE && s.ch_l_vert <= Self::DJI_RANGE);
        assert!(s.ch_r_hori >= -Self::DJI_RANGE && s.ch_r_hori <= Self::DJI_RANGE);
        assert!(s.ch_r_vert >= -Self::DJI_RANGE && s.ch_r_vert <= Self::DJI_RANGE);
        assert!(s.sw_left >= -1 && s.sw_left <= 1 && s.sw_right >= -1 && s.sw_right <= 1);

        s
    }
}
