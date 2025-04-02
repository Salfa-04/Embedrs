//!
//! # Tasks
//!

use hal::{bind_interrupts, peripherals};
use {crate::hal, embassy_executor::task};

bind_interrupts! {
    struct IntRqst {
        USART1 => hal::usart::InterruptHandler<peripherals::USART1>;
        UART5 => hal::usart::InterruptHandler<peripherals::UART5>;
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
