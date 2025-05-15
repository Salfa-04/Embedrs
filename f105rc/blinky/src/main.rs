#![no_std]
#![no_main]

use utils::prelude::*;

mod controller;
mod tasks;
mod utils;

#[embassy_executor::main]
async fn entry(s: embassy_executor::Spawner) {
    let (p,) = utils::sys_init();

    {
        let p = (p.PC13,);
        s.must_spawn(tasks::blinky::led_task(p));
    }

    {
        let p = ();
        s.must_spawn(controller::main(p));
    }
}
