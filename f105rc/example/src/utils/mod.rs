//!
//! # Utils
//!

use defmt_rtt as _;
use panic_probe as _;

mod init;
mod macros;

pub use init::sys_init;
#[allow(unused_imports)]
pub mod prelude {
    pub use ::cortex_m as ll; // Low Level
    pub use ::cortex_m_rt as rt; // Runtime
    pub use ::embassy_stm32 as hal; // HAL
    pub use ::embassy_time::Timer as T; // Timer
}

use prelude::hal::{self, bind_interrupts, peripherals};
bind_interrupts! {
    pub struct IntRqst {
        CAN1_TX => hal::can::TxInterruptHandler<peripherals::CAN1>;
        CAN1_RX0 => hal::can::Rx0InterruptHandler<peripherals::CAN1>;
        CAN1_RX1 => hal::can::Rx1InterruptHandler<peripherals::CAN1>;
        CAN1_SCE => hal::can::SceInterruptHandler<peripherals::CAN1>;

        CAN2_TX => hal::can::TxInterruptHandler<peripherals::CAN2>;
        CAN2_RX0 => hal::can::Rx0InterruptHandler<peripherals::CAN2>;
        CAN2_RX1 => hal::can::Rx1InterruptHandler<peripherals::CAN2>;
        CAN2_SCE => hal::can::SceInterruptHandler<peripherals::CAN2>;

        UART4 => hal::usart::InterruptHandler<peripherals::UART4>;
    }
}

#[::defmt::panic_handler]
fn soft_panic() -> ! {
    panic_probe::hard_fault()
}
