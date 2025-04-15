use crate::{init_ticker, tasks};

use core::sync::atomic::{AtomicBool, AtomicU8, Ordering::Relaxed};
use pid::Pid;
use tasks::{
    remote_ctrl::get_rc_data, serial_screen::screen_target, servo_ctrl::set_servo,
    vision_mv::get_mv_position,
};

/// Positions: (x, y)
const POSI: [(f32, f32); 11] = [
    (0.0, 0.0),     // unreachable
    (67.0, 203.0),  // 1
    (159.0, 209.0), // 2
    (251.0, 199.0), // 3
    (61.0, 113.0),  // 4
    (159.0, 110.0), // 5
    (257.0, 110.0), // 6
    (62.0, 20.0),   // 7
    (160.0, 16.0),  // 8
    (252.0, 19.0),  // 9
    (223.0, 175.0), // N
];

async fn vision_control(pid: &mut (Pid<f32>, Pid<f32>)) -> Option<(f32, f32)> {
    // /// VDATA[10]: Status
    // /// 0x00: Stop
    // /// 0xFF: Running
    // ///
    // /// VDATA[11]: Index of VDATA (0-9)
    // ///
    // /// VDATA[0-9]: Target Position
    // static VDATA: [AtomicU8; 12] = unsafe { core::mem::zeroed() };

    // if let Some(st) = screen_target() {
    //     VDATA[11].store(0, Relaxed); // Index Reset

    //     // Set Target Position
    //     for (i, data) in st.iter().enumerate() {
    //         VDATA[i].store(*data, Relaxed);
    //     }

    //     if st[0] != 0 {
    //         VDATA[10].store(0xFF, Relaxed); // Start
    //     } else {
    //         // Stop when the first element is 0
    //         VDATA[10].store(0, Relaxed); // Stop
    //         pid.0.reset_integral_term();
    //         pid.1.reset_integral_term();
    //         set_servo(Some((0.0, 0.0))).await;
    //     }
    // }

    match get_mv_position() {
        Some((mx, my)) => {
            // let idx = VDATA[11].load(Relaxed);
            // let (tx, ty) = POSI[VDATA[idx as usize].load(Relaxed) as usize];
            let (tx, ty) = (pid.0.setpoint, pid.1.setpoint);

            // // Check if the position is stable
            // // If the position is stable, increase the index
            // if VDATA[10].load(Relaxed) != 0 // Running
            //         && (mx as i16 - tx as i16) < 10 // Stable
            //         && (my as i16 - ty as i16) < 10 // Stable
            //         && idx < 9
            // {
            //     VDATA[11].store(idx + 1, Relaxed);
            // }

            // (pid.0.setpoint(tx as f32), pid.1.setpoint(ty as f32));

            if (mx as f32 - tx).abs() > 15.0 {
                pid.0.reset_integral_term();
            }
            if (my as f32 - ty).abs() > 15.0 {
                pid.1.reset_integral_term();
            }

            Some((
                pid.0.next_control_output(mx as f32).output,
                pid.1.next_control_output(my as f32).output,
            ))
        }

        _ => None,
    }
}

#[embassy_executor::task]
pub async fn main(_p: ()) {
    let mut t = init_ticker!(20);

    let mut angle_offset: (f32, f32) = (-0.0227272734, -5.11363649);

    let mut position = 5;

    let (mut pid_v, mut pid_p) = {
        let mut pid_v = (
            Pid::<f32>::new(0.0, 15.0), // x
            Pid::<f32>::new(0.0, 15.0), // y
        );

        let mut pid_p = (
            Pid::<f32>::new(POSI[position].0, 320.0), // x
            Pid::<f32>::new(POSI[position].1, 240.0), // y
        );

        // Speed
        pid_v.0.p(0.75, 4.0).i(0.011, 1.0).d(7.5, 12.0);
        pid_v.1.p(0.75, 4.0).i(0.011, 1.0).d(7.5, 12.0);

        // Point
        pid_p.0.p(0.2, 320.0).i(0.0, 0.0).d(0.0, 0.0);
        pid_p.1.p(0.2, 240.0).i(0.0, 0.0).d(0.0, 0.0);

        (pid_v, pid_p)
    };

    loop {
        let rc = get_rc_data().await;

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

                (
                    pid_p.0.setpoint(POSI[position].0),
                    pid_p.1.setpoint(POSI[position].1),
                );

                if rc.sw_left == -1 {
                    position = 5;
                }

                if rc.ch_l_hori == -660 {
                    position = 2;
                }

                if rc.ch_l_hori == 660 {
                    position = 6;
                }

                if rc.ch_l_vert == 660 {
                    position = 9;
                }

                if rc.ch_l_vert == -660 {
                    position = 10;
                }

                if let Some(p_out) = vision_control(&mut pid_p).await {
                    let (v_out_x, v_out_y) = (
                        pid_v.0.next_control_output(-p_out.0).output,
                        pid_v.1.next_control_output(-p_out.1).output,
                    );

                    set_servo(Some(
                        (v_out_x + angle_offset.0, v_out_y + angle_offset.1).limit(),
                    ))
                    .await;
                } else {
                    // set_servo(Some((0.0, 0.0))).await;
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

trait ClearPID {
    fn clear(&mut self);
}

impl ClearPID for (Pid<f32>, Pid<f32>) {
    fn clear(&mut self) {
        (self.0.reset_integral_term(), self.1.reset_integral_term());
        let _ = self.0.next_control_output(self.0.setpoint);
        let _ = self.1.next_control_output(self.1.setpoint);
    }
}

trait ServoLimit {
    const LIMIT: (f32, f32);
    fn limit(&self) -> (f32, f32);
}

impl ServoLimit for (f32, f32) {
    const LIMIT: (f32, f32) = (15.0, 15.0);

    fn limit(&self) -> (f32, f32) {
        (
            self.0.clamp(-Self::LIMIT.0, Self::LIMIT.0),
            self.1.clamp(-Self::LIMIT.1, Self::LIMIT.1),
        )
    }
}
