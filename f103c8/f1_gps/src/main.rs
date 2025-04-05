#![no_std]
#![no_main]

use utils::prelude::*;

mod tasks;
mod utils;

#[embassy_executor::main]
async fn entry(s: embassy_executor::Spawner) {
    let (p,) = utils::sys_init();

    {
        // !TASK: led_task
        let p = (p.PC13,);
        s.must_spawn(tasks::led_task(p));
    }

    {
        // !TASK: gps_task
        let p = (p.USART3, p.PB11, p.DMA1_CH3);
        s.must_spawn(tasks::gps_task(p));
    }
}
