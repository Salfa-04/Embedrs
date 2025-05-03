//!
//! # Tasks
//!

#![allow(unused_imports)]

pub mod remote_ctrl {
    mod rc_task;
    mod sbus;

    // pub use rc_task::DjiSBusPacket;
    pub use rc_task::get_rc_data;
    pub use rc_task::rc_task;
}

pub mod blinky {
    mod led_task;

    pub use led_task::led_task;
}
