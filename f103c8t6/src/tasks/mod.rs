//!
//! # Tasks
//!

use hal::{bind_interrupts, peripherals};
use {crate::hal, embassy_executor::task};

mod gps_task;
pub use gps_task::gps_task;

bind_interrupts! {
    struct IntRqst {
        USART3 => hal::usart::InterruptHandler<peripherals::USART3>;
    }
}

mod led_task;
pub use led_task::led_task;
