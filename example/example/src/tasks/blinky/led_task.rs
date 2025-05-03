//!
//! # LED Task
//!

use crate::{hal, init_ticker};
use hal::{gpio, peripherals};

use gpio::{Level, Output as OP, Speed};
use peripherals::PC13;

#[embassy_executor::task]
pub async fn led_task(p: (PC13,)) -> ! {
    let mut t = init_ticker!(150); // ms

    let mut led = OP::new(p.0, Level::Low, Speed::Low);

    loop {
        led.toggle();
        t.next().await;
    }
}
