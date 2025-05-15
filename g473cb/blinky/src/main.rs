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

    {
        let mut config = hal::usart::Config::default();
        config.baudrate = 4_000_000;

        let uart = hal::usart::Uart::new_with_de(
            p.USART3,
            p.PB11,
            p.PB10,
            utils::IntRqst,
            p.PB14,
            p.DMA1_CH1,
            p.DMA1_CH2,
            config,
        )
        .unwrap();
    }
}
