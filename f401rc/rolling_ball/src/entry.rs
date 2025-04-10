use crate::{init_ticker, tasks};

use core::sync::atomic::{AtomicU8, Ordering::Relaxed};
use pid::Pid;
use tasks::{
    remote_ctrl::get_rc_data, serial_screen::screen_target, servo_ctrl::set_servo,
    vision_mv::get_mv_position,
};

/// Positions: (x, y)
const POSI: [(f32, f32); 10] = [
    (0.0, 0.0),
    (0.0, 135.0),     // 1
    (135.0, 135.0),   // 2
    (135.0, 0.0),     // 3
    (135.0, -135.0),  // 4
    (0.0, -135.0),    // 5
    (-135.0, -135.0), // 6
    (-135.0, 0.0),    // 7
    (-135.0, 135.0),  // 8
    (-135.0, 0.0),    // 9
];

async fn vision_control(pid: &mut (Pid<f32>, Pid<f32>)) -> Option<(f32, f32)> {
    /// VDATA[10]: Status
    /// 0x00: Stop
    /// 0xFF: Running
    ///
    /// VDATA[11]: Index of VDATA (0-9)
    ///
    /// VDATA[0-9]: Target Position
    static VDATA: [AtomicU8; 12] = unsafe { core::mem::zeroed() };

    if let Some(st) = screen_target() {
        VDATA[11].store(0, Relaxed); // Index Reset

        // Set Target Position
        for (i, data) in st.iter().enumerate() {
            VDATA[i].store(*data, Relaxed);
        }

        if st[0] != 0 {
            VDATA[10].store(0xFF, Relaxed); // Start
        } else {
            // Stop when the first element is 0
            VDATA[10].store(0, Relaxed); // Stop
            pid.0.reset_integral_term();
            pid.1.reset_integral_term();
            set_servo(Some((0.0, 0.0))).await;
        }
    }

    match get_mv_position() {
        Some((mx, my)) => {
            let idx = VDATA[11].load(Relaxed);
            let (tx, ty) = POSI[VDATA[idx as usize].load(Relaxed) as usize];

            // Check if the position is stable
            // If the position is stable, increase the index
            if VDATA[10].load(Relaxed) != 0 // Running
                    && (mx as i16 - tx as i16) < 10 // Stable
                    && (my as i16 - ty as i16) < 10 // Stable
                    && idx < 9
            {
                VDATA[11].store(idx + 1, Relaxed);
            }

            (pid.0.setpoint(tx as f32), pid.1.setpoint(ty as f32));

            Some((
                pid.0.next_control_output(mx as f32).output,
                pid.1.next_control_output(my as f32).output,
            ))
        }

        None => None,
    }
}

#[embassy_executor::task]
pub async fn main(_p: ()) {
    let mut t = init_ticker!(20);

    let mut pid = (
        Pid::<f32>::new(0.0, 0.0), // x
        Pid::<f32>::new(0.0, 0.0), // y
    );

    pid.0.p(0.0, 0.0).i(0.0, 0.0).d(0.0, 0.0);
    pid.1.p(0.0, 0.0).i(0.0, 0.0).d(0.0, 0.0);

    loop {
        let rc = get_rc_data().await;

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
                if let Some(p) = vision_control(&mut pid).await {
                    set_servo(Some(p)).await;
                } else {
                    set_servo(None).await;
                }
            }

            _ => unreachable!(),
        }

        t.next().await;
    }
}
