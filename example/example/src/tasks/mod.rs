//!
//! # Tasks
//!

#![allow(unused_imports)]

pub mod blinky {
    mod led_task;

    pub use led_task::led_task;
}
