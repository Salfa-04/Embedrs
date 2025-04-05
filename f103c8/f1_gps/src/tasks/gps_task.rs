// use crate::hal;
use crate::hal::{peripherals, usart};

use defmt::{debug, error, info};
use peripherals::{DMA1_CH3, PB11, USART3};
use type_def::GpsRMC;
use usart::{Config as C, UartRx as P};

use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;

mod type_def;

static GPS_DATA: Mutex<ThreadModeRawMutex, GpsRMC> = Mutex::new(GpsRMC::new());

async fn read_gps_data(buffer: &[u8]) {
    for data in buffer.split(|&x| x == b'$') {
        if !data.ends_with(b"\r\n") {
            continue;
        }

        let data = data.trim_ascii();
        if data.get(2..5) != Some(b"RMC") {
            continue;
        };

        // id   ,          ,V     ,         ,    ,          ,    ,     ,      ,      ,  ,   ,N   ,V*35
        // 0    ,1         ,2     ,3        ,4   ,5         ,6   ,7    ,8     ,9     ,10,11 ,12  ,13
        // GPRMC,235316.000,A     ,2959.9925,S   ,12000.0090,E   ,0.009,75.020,020711,  ,   ,A   ,V*45\r\n
        // --RMC,UTCtime   ,status,lat      ,uLat,lon       ,uLon,spd  ,cog   ,date  ,mv,mvE,mode,nS*CS\r\n

        // pub struct GpsRMC {
        // 2    pub status: char, // A=active, V=void
        // 3    pub latitude: f32,
        // 4    pub ulatitude: char, // N=north, S=south
        // 5    pub longitude: f32,
        // 6    pub ulongitude: char, // E=east, W=west
        // 7    pub speed: f32,       // knots
        // }

        for (i, raw) in data.split(|&x| x == b',').enumerate() {
            let Ok(data) = core::str::from_utf8(raw) else {
                error!("Invalid Data: {:?}", raw);
                continue;
            };

            debug!("{}: {}", i, data);

            let mut gps_data = GPS_DATA.lock().await;

            match i {
                2 => {
                    if raw.len() != 1 {
                        continue;
                    }
                    gps_data.status = raw[0] as char;
                }
                3 => {
                    if let Ok(data) = data.parse::<f32>() {
                        gps_data.latitude = data;
                    }
                }
                4 => {
                    if raw.len() != 1 {
                        continue;
                    }
                    gps_data.ulatitude = raw[0] as char;
                }
                5 => {
                    if let Ok(data) = data.parse::<f32>() {
                        gps_data.longitude = data;
                    }
                }
                6 => {
                    if raw.len() != 1 {
                        continue;
                    }
                    gps_data.ulongitude = raw[0] as char;
                }
                7 => {
                    if let Ok(data) = data.parse::<f32>() {
                        gps_data.speed = data;
                    }
                }

                _ => {}
            }
        }
    }
}

#[super::task]
pub async fn gps_task(p: (USART3, PB11, DMA1_CH3)) -> ! {
    // let mut led = OP::new(p.0, Level::High, Speed::Low);
    let mut config = C::default();
    config.baudrate = 4800;

    let mut rx = P::new(p.0, super::IntRqst, p.1, p.2, config)
        .inspect_err(|e| error!("USART Init Error: {:?}", e))
        .unwrap();

    let mut buffer = [0u8; 512];

    loop {
        match rx.read_until_idle(&mut buffer).await {
            Ok(x) => read_gps_data(&buffer[..x]).await,
            Err(e) => error!("UART Read Error: {:?}", e),
        }

        // info!("{:?}", *GPS_DATA.lock().await);
    }
}
