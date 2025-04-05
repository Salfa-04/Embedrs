#![no_std]
#![no_main]

use utils::prelude::*;

// mod test_task;

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
        let p = (p.USART1, p.PA10, p.DMA2_CH2);
        s.must_spawn(tasks::vision_mv::mv_task(p));
    }

    {
        let p = ();
        s.must_spawn(main(p));
    }
}

#[embassy_executor::task]
async fn main(_p: ()) {
    use {
        pid::Pid,
        tasks::{remote_ctrl::get_rc_data, servo_ctrl::set_servo, vision_mv::get_mv_position},
    };

    let mut t = init_ticker!(1);

    let mut pid = (
        Pid::<f32>::new(0.0, 0.0), // x
        Pid::<f32>::new(0.0, 0.0), // y
    );

    pid.0.p(0.0, 0.0).i(0.0, 0.0).d(0.0, 0.0);
    pid.1.p(0.0, 0.0).i(0.0, 0.0).d(0.0, 0.0);

    loop {
        let rc = get_rc_data().await;

        if rc.sw_right == -1 {
            // Remote Control

            (pid.0.reset_integral_term(), pid.1.reset_integral_term());

            set_servo((
                (rc.ch_r_hori as f32 * 135f32 / 660f32),
                (rc.ch_r_vert as f32 * 135f32 / 660f32),
            ))
            .await;
        } else {
            // Vision Control

            let (mx, my) = get_mv_position().await;

            set_servo((
                pid.0.next_control_output(mx).output,
                pid.1.next_control_output(my).output,
            ))
            .await;
        }

        t.next().await;
    }
}
