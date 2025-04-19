use crate::{init_ticker, tasks};

use blocking_mutex::raw::ThreadModeRawMutex as RM;
use core::sync::atomic::{AtomicBool, Ordering::Relaxed};
use embassy_sync::{blocking_mutex, mutex::Mutex};
use impls::{ClearPID, ServoLimit};
use tasks::{remote_ctrl::get_rc_data, servo_ctrl::set_servo};
use vision_control::{vision_control_set, vision_control_update};

pub mod consts;

mod impls;
mod inits;
mod vision_control;

static POSI_NUM: Mutex<RM, usize> = Mutex::new(5);

#[embassy_executor::task]
pub async fn main(_p: ()) {
    let mut t = init_ticker!(20);

    let mut angle_offset: (f32, f32) = (-0.0227272734, -5.11363649);

    let (mut pid_v, mut pid_p) = inits::pid_init().await;

    loop {
        let rc = get_rc_data().await;

        #[cfg(debug_assertions)]
        {
            static BREAKPOINT_ONCE: AtomicBool = AtomicBool::new(true);
            if rc.sw_left == 1 && BREAKPOINT_ONCE.load(Relaxed) {
                let _breakpoint_here = ();
                BREAKPOINT_ONCE.store(false, Relaxed);
            } else if rc.sw_left != 1 {
                BREAKPOINT_ONCE.store(true, Relaxed);
            }
        }

        match rc.sw_right {
            0 => {
                // Remote Control
                (pid_v.clear(), pid_p.clear());

                let x = rc.ch_r_hori as f32 * 15f32 / 660f32;
                let y = rc.ch_r_vert as f32 * 15f32 / 660f32;

                {
                    static RECORD_ONCE: AtomicBool = AtomicBool::new(true);
                    if rc.sw_left == -1 && RECORD_ONCE.load(Relaxed) {
                        angle_offset = (x, y);
                        RECORD_ONCE.store(false, Relaxed);
                    } else if rc.sw_left != -1 {
                        RECORD_ONCE.store(true, Relaxed);
                    } else {
                        t.next().await;
                        continue;
                    }
                }

                // defmt::info!("[x, y]: {:?}", (x, y));

                set_servo(Some(
                    (x + angle_offset.0 as f32, y + angle_offset.1 as f32).limit(),
                ))
                .await;
            }

            1 => {
                // Vision Control
                vision_control_set(&rc, &mut pid_p).await;

                if let Some((out_x, out_y)) = vision_control_update(&mut pid_p, &mut pid_v).await {
                    set_servo(Some(
                        (out_x + angle_offset.0, out_y + angle_offset.1).limit(),
                    ))
                    .await;
                } else {
                    set_servo(None).await;
                }
            }

            _ => {
                // Stop Servo
                (pid_v.clear(), pid_p.clear());

                if rc.sw_left == -1 {
                    angle_offset = (0.0, 0.0);
                }

                set_servo(None).await;
            }
        }

        t.next().await;
    }
}
