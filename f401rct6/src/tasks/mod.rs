//!
//! # Tasks
//!

use hal::{bind_interrupts, peripherals};
use {crate::hal, embassy_executor::task};

bind_interrupts! {
    struct IntRqst {
        USART1 => hal::usart::InterruptHandler<peripherals::USART1>;
        USART6 => hal::usart::InterruptHandler<peripherals::USART6>;
    }
}

pub mod remote_ctrl {
    use super::{IntRqst, task};

    mod rc_task;
    mod sbus;

    pub use rc_task::DjiSBusPacket;
    pub use rc_task::get_rc_data;
    pub use rc_task::rc_task;
}

pub mod servo_ctrl {
    use super::{IntRqst, task};

    mod dbg_task;
    mod pwm_task;
    mod pwm_utils;

    pub use dbg_task::dbg_task;
    pub use pwm_task::pwm_task;
    pub use pwm_task::set_servo;
}

mod led_task;
pub use led_task::led_task;
