#![no_std]
#![no_main]

use utils::prelude::*;

mod test_task;

mod tasks;
mod utils;

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
        let p = ();
        s.must_spawn(main(p));
    }
}

#[embassy_executor::task]
async fn main(_p: ()) {
    let mut t = init_ticker!(100);

    loop {
        let rc = tasks::remote_ctrl::get_rc_data().await;
        tasks::servo_ctrl::set_servo((
            (rc.ch_r_hori as f32 * 135f32 / 660f32) as i16,
            (rc.ch_r_vert as f32 * 135f32 / 660f32) as i16,
        ))
        .await;

        t.next().await;
    }
}
