//!
//! # Utils
//!

use defmt_rtt as _;
use panic_probe as _;

mod config;
mod init;
mod macros;

#[allow(unused_imports)]
pub mod prelude {
    pub use ::cortex_m as ll; // Low Level
    pub use ::cortex_m_rt as rt; // Runtime
    pub use ::embassy_stm32 as hal; // HAL
    pub use ::embassy_time::Timer as T; // Timer

    pub use super::config::CONFIG; // Config
}

pub use init::sys_init;

#[::defmt::panic_handler]
fn soft_panic() -> ! {
    panic_probe::hard_fault();
}
