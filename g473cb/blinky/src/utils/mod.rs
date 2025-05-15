//!
//! # Utils
//!

#![allow(unused_imports)]
use defmt_rtt as _;
use panic_probe as _;

pub use binding::IntRqst;
pub use init::sys_init;

mod binding;
mod init;
mod macros;

pub mod prelude {
    pub use ::bitfield_struct::bitfield; // Bitfield
    pub use ::cortex_m as ll; // Low Level
    pub use ::cortex_m_rt as rt; // Runtime
    pub use ::embassy_sync as sync; // Sync
    pub use ::embassy_time::Timer as T; // Timer

    pub use ::embassy_stm32 as hal; // HAL
}

#[::defmt::panic_handler]
fn soft_panic() -> ! {
    panic_probe::hard_fault()
}
