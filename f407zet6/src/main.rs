#![no_std]
#![no_main]

use utils::prelude::*;

mod tasks;
mod utils;

#[embassy_executor::main]
async fn entry(s: embassy_executor::Spawner) {
    let (p,) = utils::sys_init();

    // {
    //     let p = (p.PC7,);
    //     s.must_spawn(tasks::led_task(p));
    // }

    // {
    //     let p = (p.TIM3, p.PA6, p.PA7);
    //     s.must_spawn(tasks::servo_ctrl::pwm_task(p));
    // }

    {
        let p = (p.USART3, p.PC11, p.DMA1_CH1);
        s.must_spawn(tasks::remote_ctrl::rc_task(p));
    }

    // {
    //     let p = (p.USART1, p.PA10, p.DMA2_CH2);
    //     s.must_spawn(tasks::servo_ctrl::dbg_task(p));
    // }

    // {
    //     let p = ();
    //     s.must_spawn(main(p));
    // }
}

#[embassy_executor::task]
async fn main(_p: ()) {}
