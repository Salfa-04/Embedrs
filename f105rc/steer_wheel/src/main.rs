#![no_std]
#![no_main]

use controller::main;
use utils::prelude::*;

mod controller;
mod tasks;
mod utils;

// mod test_task;

#[embassy_executor::main]
async fn entry(s: embassy_executor::Spawner) {
    let (p,) = utils::sys_init();

    {
        let p = ();
        s.must_spawn(main(p));
    }

    {
        use hal::can::{self, Fifo, filter::Mask16};

        let mut can1 = can::Can::new(p.CAN1, p.PB8, p.PB9, crate::utils::IntRqst);
        // let can2 = Can::new(p.CAN2, p.PB12, p.PB13, crate::utils::IntRqst);

        can1.modify_config()
            .set_automatic_retransmit(true)
            .set_loopback(false)
            .set_silent(false)
            .set_bitrate(1_000_000);

        can1.modify_filters().enable_bank(
            0,
            Fifo::Fifo0,
            can::filter::BankConfig::Mask16([Mask16::accept_all(); 2]),
        );

        can1.enable().await;

        can1.buffered(txb, rxb);

        can1.write(&can::Frame::new_standard(0, &[0]).unwrap())
            .await;
    }
}
