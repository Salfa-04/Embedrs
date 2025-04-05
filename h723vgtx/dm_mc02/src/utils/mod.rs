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

#[::defmt::panic_handler]
fn soft_panic() -> ! {
    panic_probe::hard_fault()
}
