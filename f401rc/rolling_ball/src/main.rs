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
        let p = (p.PC7,);
        s.must_spawn(tasks::led_task(p));
    }

    {
        let p = (p.TIM3, p.PA6, p.PA7);
        s.must_spawn(tasks::servo_ctrl::pwm_task(p));
    }

    {
        let p = (p.USART6, p.PA12, p.DMA2_CH1);
        s.must_spawn(tasks::remote_ctrl::rc_task(p));
    }

    {
        let p = (p.USART2, p.PA3, p.PA2, p.DMA1_CH5, p.DMA1_CH6);
        s.must_spawn(tasks::serial_screen::screen_task(p));
    }

    {
        let p = (p.USART1, p.PA10, p.DMA2_CH2);
        s.must_spawn(tasks::vision_mv::mv_task(p));
    }

    {
        let p = ();
        s.must_spawn(controller::main(p));
    }
}
