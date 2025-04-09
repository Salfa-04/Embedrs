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
        let p = (p.USART2, p.PA3, p.PA2, p.DMA1_CH5, p.DMA1_CH6);
        s.must_spawn(tasks::serial_screen::screen_task(p));
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
        tasks::{
            remote_ctrl::get_rc_data, serial_screen::get_screen_fb, servo_ctrl::set_servo,
            vision_mv::get_mv_position,
        },
    };

    let mut t = init_ticker!(20);

    let mut pid = (
        Pid::<f32>::new(0.0, 0.0), // x
        Pid::<f32>::new(0.0, 0.0), // y
    );

    pid.0.p(0.0, 0.0).i(0.0, 0.0).d(0.0, 0.0);
    pid.1.p(0.0, 0.0).i(0.0, 0.0).d(0.0, 0.0);

    loop {
        let rc = get_rc_data().await;
        let ss = get_screen_fb().await;

        match rc.sw_right {
            -1 => {
                // Stop Servo
                pid.0.reset_integral_term();
                pid.1.reset_integral_term();
                set_servo(None).await;
            }

            0 => {
                // Remote Control
                pid.0.reset_integral_term();
                pid.1.reset_integral_term();
                set_servo(Some((
                    (rc.ch_r_hori as f32 * 135f32 / 660f32),
                    (rc.ch_r_vert as f32 * 135f32 / 660f32),
                )))
                .await;
            }

            1 => {
                // Vision Control

                match get_mv_position() {
                    Some((mx, my)) => {
                        set_servo(Some((
                            pid.0.next_control_output(mx).output,
                            pid.1.next_control_output(my).output,
                        )))
                        .await
                    }

                    None => continue,
                }
            }

            _ => unreachable!(),
        }

        t.next().await;
    }
}
