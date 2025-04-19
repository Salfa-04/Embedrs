use crate::{hal, init_ticker};
use hal::{gpio, peripherals};

use gpio::OutputOpenDrain as P;
use gpio::{Level, Speed};

#[embassy_executor::task]
pub async fn led_task(p: (peripherals::PC7,)) -> ! {
    let mut t = init_ticker!(150);

    let mut led = P::new(p.0, Level::High, Speed::Low);

    loop {
        led.toggle();
        t.next().await;
    }
}
