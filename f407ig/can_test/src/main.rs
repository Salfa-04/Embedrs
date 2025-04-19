#![no_std]
#![no_main]

use utils::prelude::*;

mod controller;
mod tasks;
mod utils;

// mod test_task;

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
        use hal::can::{
            Can, RxBuf, StandardId, TxBuf,
            filter::{BankConfig, Mask32},
        };

        // let mut can2 = hal::can::Can::new(p.CAN2, p.PB5, p.PB6, utils::IntRqst);

        let mut can1 = {
            let mut can = Can::new(p.CAN1, p.PD0, p.PD1, utils::IntRqst);

            can.modify_config()
                .set_silent(false)
                .set_loopback(false)
                .set_automatic_retransmit(true)
                .set_bitrate(1_000_000);

            can.modify_filters().enable_bank(
                0,
                embassy_stm32::can::Fifo::Fifo0,
                BankConfig::Mask32(Mask32::frames_with_std_id(
                    StandardId::new(0x200).unwrap(),
                    StandardId::MAX,
                )),
            );

            can
        };

        // let rx_buf = RxBuf::new();
        // let tx_buf = TxBuf::new();

        let (rx_buf, tx_buf) = {
            use static_cell::StaticCell;

            static RX_BUF: StaticCell<RxBuf<32>> = StaticCell::new();
            static TX_BUF: StaticCell<TxBuf<32>> = StaticCell::new();
            (RX_BUF.init(RxBuf::new()), TX_BUF.init(TxBuf::new()))
        };

        can1.enable().await;
        can1.buffered(tx_buf, rx_buf);

        loop {
            let a = can1.read().await;
            defmt::info!("CAN1: {:?}", a);
        }
    }
}
