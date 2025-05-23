#![no_std]
#![no_main]

use utils::prelude::*;

use hal::can::{
    Can, RxBuf, TxBuf,
    filter::{BankConfig, Mask32},
};

mod controller;
mod tasks;
mod utils;

#[embassy_executor::main]
async fn entry(s: embassy_executor::Spawner) {
    let (p,) = utils::sys_init();

    {
        let p = (p.TIM5, p.PH12, p.PH11, p.PH10);
        s.must_spawn(tasks::led_task(p));
    }

    {
        let p = ();
        s.must_spawn(controller::main(p));
    }

    {
        let (mut can1, mut can2) = {
            let mut can1 = Can::new(p.CAN1, p.PD0, p.PD1, utils::IntRqst);
            let mut can2 = Can::new(p.CAN2, p.PB5, p.PB6, utils::IntRqst);

            can1.modify_config()
                .set_silent(false)
                .set_loopback(false)
                .set_automatic_retransmit(true)
                .set_bitrate(1_000_000);

            can1.modify_filters().enable_bank(
                0,
                embassy_stm32::can::Fifo::Fifo0,
                BankConfig::Mask32(Mask32::accept_all()),
            );

            can2.modify_config()
                .set_silent(false)
                .set_loopback(false)
                .set_automatic_retransmit(true)
                .set_bitrate(1_000_000);

            can2.modify_filters().enable_bank(
                0,
                embassy_stm32::can::Fifo::Fifo0,
                BankConfig::Mask32(Mask32::accept_all()),
            );

            (can1, can2)
        };

        let (rx_buf_1, tx_buf_1) = {
            use static_cell::StaticCell;
            static RX_BUF: StaticCell<RxBuf<32>> = StaticCell::new();
            static TX_BUF: StaticCell<TxBuf<32>> = StaticCell::new();
            (RX_BUF.init(RxBuf::new()), TX_BUF.init(TxBuf::new()))
        };

        let (rx_buf_2, tx_buf_2) = {
            use static_cell::StaticCell;
            static RX_BUF: StaticCell<RxBuf<32>> = StaticCell::new();
            static TX_BUF: StaticCell<TxBuf<32>> = StaticCell::new();
            (RX_BUF.init(RxBuf::new()), TX_BUF.init(TxBuf::new()))
        };

        (can1.enable().await, can2.enable().await);
        can1.buffered(tx_buf_1, rx_buf_1);
        can2.buffered(tx_buf_2, rx_buf_2);

        s.must_spawn(can_task(can1));
        s.must_spawn(can_task(can2));

        loop {
            // prevent to drop cans
            T::after_secs(60).await
        }
    }
}

#[bitfield(u64, clone = false, debug = false, default = false, defmt = true)]
struct M2006 {
    angle: u16,
    speed: i16,
    current: i16,
    __: u16,
}

#[embassy_executor::task(pool_size = 2)]
async fn can_task(mut can: embassy_stm32::can::Can<'static>) {
    // use hal::can::Frame;

    // let frame = Frame::new_standard(0x6FF, &[0xE9, 0x00, 0x3, 0xCC]).unwrap();

    loop {
        // can.write(&frame).await;

        let frame = match can.read().await {
            Ok(x) => x.frame,
            Err(e) => {
                defmt::error!("ERROR: {:?}", e);
                continue;
            }
        };

        let data = frame.data();

        // let a = unsafe { *(frame.data().as_ptr() as *const u64) };

        let m2006 = M2006::new()
            .with_angle(((data[0] as u16) << 8 | (data[1] as u16)) as _)
            .with_speed(((data[2] as u16) << 8 | (data[3] as u16)) as _)
            .with_current(((data[4] as u16) << 8 | (data[5] as u16)) as _);

        defmt::info!("ID={:x}, DATA={}", defmt::Debug2Format(&frame.id()), m2006);

        // T::after_millis(100).await;
    }
}
