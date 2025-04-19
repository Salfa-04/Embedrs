use super::{POSI_NUM, impls::PidTd};
use crate::tasks::{
    remote_ctrl::DjiSBusPacket,
    serial_screen::screen_target,
    vision_mv::{get_mv_position, get_mv_positions},
};

pub async fn vision_control_set(rc: &DjiSBusPacket, pid_p: &mut PidTd<f32>) {
    let mut position = POSI_NUM.lock().await;
    pid_p.setpoint(get_mv_positions().await[*position]);

    if let Some(sc) = screen_target() {
        match sc {
            1 => *position = 1,
            2 => *position = 2,
            3 => *position = 3,
            4 => *position = 4,
            5 => *position = 5,
            6 => *position = 6,
            7 => *position = 7,
            8 => *position = 8,
            9 => *position = 9,
            _ => {}
        }
    }

    defmt::debug!("Vision MV: {:?}", *position);

    if rc.sw_left == -1 {
        *position = 5;
    }

    if rc.ch_l_hori == -660 {
        *position = 2;
    }

    if rc.ch_l_hori == 660 {
        *position = 6;
    }

    if rc.ch_l_vert == 660 {
        *position = 9;
    }
}

pub async fn vision_control_update(
    pid_p: &mut PidTd<f32>,
    pid_v: &mut PidTd<f32>,
) -> Option<(f32, f32)> {
    match get_mv_position() {
        Some((mx, my)) => {
            // let (tx, ty) = (pid_p.0.setpoint, pid_p.1.setpoint);

            // if (mx as f32 - tx).abs() > 15.0 {
            //     // pid_v.0.reset_integral_term();
            //     pid_v.0.set_integral_term(PID_V_KI);
            // } else {
            //     pid_v.0.set_integral_term(PID_V_KI * 6.0 / 5.0);
            // }
            // if (my as f32 - ty).abs() > 15.0 {
            //     // pid_v.1.reset_integral_term();
            //     pid_v.1.set_integral_term(PID_V_KI);
            // } else {
            //     pid_v.1.set_integral_term(PID_V_KI * 6.0 / 5.0);
            // }

            // if (mx as f32 - tx).abs() > 10.0 {
            //     pid_v.0.reset_integral_term();
            // }
            // if (my as f32 - ty).abs() > 10.0 {
            //     pid_v.1.reset_integral_term();
            // }

            let (p_out_x, p_out_y) = pid_p.calculate((mx as f32, my as f32));
            Some(pid_v.calculate((-p_out_x, -p_out_y)))
        }

        _ => None,
    }
}
