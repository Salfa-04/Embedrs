#![no_std]
#![no_main]

#[allow(unused_imports)]
use utils::prelude::*;

mod tasks;
mod utils;

#[embassy_executor::main]
async fn entry(s: embassy_executor::Spawner) {
    let (p,) = utils::sys_init();

    {
        let p = (p.UART5, p.PD2, p.DMA1_CH3);
        s.must_spawn(tasks::remote_ctrl::rc_task(p));
    }

    loop {
        let rc = tasks::remote_ctrl::get_rc_data().await;
        defmt::info!("{:?}", rc);

        T::after_millis(50).await;
    }
}
