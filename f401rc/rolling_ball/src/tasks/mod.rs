//!
//! # Tasks
//!

use crate::utils::IntRqst;
use embassy_executor::task;

pub mod remote_ctrl {
    use super::{IntRqst, task};

    mod rc_task;
    mod sbus;

    // pub use rc_task::DjiSBusPacket;
    pub use rc_task::get_rc_data;
    pub use rc_task::rc_task;
}

pub mod servo_ctrl {
    use super::task;

    mod pwm_task;
    mod pwm_utils;

    pub use pwm_task::pwm_task;
    pub use pwm_task::set_servo;
}

pub mod vision_mv {
    use super::{IntRqst, task};

    mod mv_task;
    // mod mv_utils;

    pub use mv_task::get_mv_position;
    pub use mv_task::mv_task;
}

pub mod serial_screen {
    use super::{IntRqst, task};

    mod screen_task;

    pub use screen_task::get_screen_fb;
    pub use screen_task::screen_task;
}

mod led_task;
pub use led_task::led_task;
