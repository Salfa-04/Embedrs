//!
//! # LED Task
//!

use crate::{hal, init_ticker, utils::IntRqst};
use embassy_executor::{SendSpawner, task};
use hal::{gpio, peripherals, timer, usart};

use crate::tasks::remote_ctrl::rc_task as rc_test;
use defmt::{error, info};

///
/// ## Example
/// ```rust
/// {
///     let p = (
///         (p.PC7,),                                          // led
///         (p.TIM3, p.PA6, p.PA7, p.PB0, p.PB1),              // servo
///         (p.USART6, p.PA12, p.DMA2_CH1),                    // rc
///         (p.USART1, p.PA10, p.PA9, p.DMA2_CH2, p.DMA2_CH7), // u1
///         (p.USART2, p.PA3, p.PA2, p.DMA1_CH5, p.DMA1_CH6),  // u2
///     );
///
///     s.must_spawn(test_task::test_task(s.make_send(), p));
/// }
///
/// ```
///

#[task]
pub async fn test_task(
    s: SendSpawner,
    p: (
        (peripherals::PC7,),
        (
            peripherals::TIM3,
            peripherals::PA6,
            peripherals::PA7,
            peripherals::PB0,
            peripherals::PB1,
        ),
        (
            peripherals::USART6,
            peripherals::PA12,
            peripherals::DMA2_CH1,
        ),
        (
            peripherals::USART1,
            peripherals::PA10,
            peripherals::PA9,
            peripherals::DMA2_CH2,
            peripherals::DMA2_CH7,
        ),
        (
            peripherals::USART2,
            peripherals::PA3,
            peripherals::PA2,
            peripherals::DMA1_CH5,
            peripherals::DMA1_CH6,
        ),
    ),
) {
    s.must_spawn(led_test(p.0));
    s.must_spawn(pwm_test(p.1));
    s.must_spawn(rc_test(p.2));
    s.must_spawn(u1_test(p.3));
    s.must_spawn(u2_test(p.4));
}

use usart::{Config, Uart};

// pa10pa11 pa3pa2
#[task]
async fn u1_test(
    p: (
        peripherals::USART1,
        peripherals::PA10,
        peripherals::PA9,
        peripherals::DMA2_CH2,
        peripherals::DMA2_CH7,
    ),
) {
    let mut config = Config::default();
    config.baudrate = 115200;
    config.rx_pull = gpio::Pull::Up;

    let mut u = Uart::new(p.0, p.1, p.2, IntRqst, p.4, p.3, config).unwrap();
    let mut buffer = [0u8; 64];

    loop {
        match u.read_until_idle(&mut buffer).await {
            Ok(x) => {
                if let Err(e) = u.write(&buffer).await {
                    error!("U1 Write Error: {:?}", e);
                };
                info!("U1 Read: {:?}", &buffer[..x]);
            }
            Err(e) => error!("U1 Read Error: {:?}", e),
        };
    }
}

#[task]
async fn u2_test(
    p: (
        peripherals::USART2,
        peripherals::PA3,
        peripherals::PA2,
        peripherals::DMA1_CH5,
        peripherals::DMA1_CH6,
    ),
) {
    let mut config = Config::default();
    config.baudrate = 115200;
    config.rx_pull = gpio::Pull::Up;

    let mut u = Uart::new(p.0, p.1, p.2, IntRqst, p.4, p.3, config).unwrap();
    let mut buffer = [0u8; 64];

    loop {
        match u.read_until_idle(&mut buffer).await {
            Ok(x) => {
                if let Err(e) = u.write(&buffer).await {
                    error!("U2 Write Error: {:?}", e);
                };
                info!("U2 Read: {:?}", &buffer[..x]);
            }
            Err(e) => error!("U2 Read Error: {:?}", e),
        };
    }
}

use gpio::OutputType;
use hal::time::hz;
use timer::low_level::CountingMode::EdgeAlignedUp;
use timer::simple_pwm::PwmPin;
use timer::simple_pwm::SimplePwm;

#[task]
async fn pwm_test(
    p: (
        peripherals::TIM3,
        peripherals::PA6,
        peripherals::PA7,
        peripherals::PB0,
        peripherals::PB1,
    ),
) {
    let mut t = init_ticker!(20);

    let pin_1 = PwmPin::new_ch1(p.1, OutputType::PushPull);
    let pin_2 = PwmPin::new_ch2(p.2, OutputType::PushPull);
    let pin_3 = PwmPin::new_ch3(p.3, OutputType::PushPull);
    let pin_4 = PwmPin::new_ch4(p.4, OutputType::PushPull);

    let pwm = SimplePwm::new(
        p.0,
        Some(pin_1),
        Some(pin_2),
        Some(pin_3),
        Some(pin_4),
        hz(50),
        EdgeAlignedUp,
    );

    let max = pwm.max_duty_cycle();
    let channels = pwm.split();
    let mut ch_1 = channels.ch1;
    let mut ch_2 = channels.ch2;
    let mut ch_3 = channels.ch3;
    let mut ch_4 = channels.ch4;
    (ch_1.enable(), ch_2.enable(), ch_3.enable(), ch_4.enable());

    let mut out = 0.025f32;

    loop {
        ch_1.set_duty_cycle((max as f32 * out) as u16);
        ch_2.set_duty_cycle((max as f32 * out) as u16);
        ch_3.set_duty_cycle((max as f32 * out) as u16);
        ch_4.set_duty_cycle((max as f32 * out) as u16);

        out += 0.001;
        if out > 0.125f32 {
            out = 0.025f32;
        }

        t.next().await;
    }
}

use gpio::OutputOpenDrain as P;
use gpio::{Level, Speed};

#[task]
async fn led_test(p: (peripherals::PC7,)) {
    let mut t = init_ticker!(130);

    let mut led = P::new(p.0, Level::High, Speed::Low);

    loop {
        led.toggle();
        t.next().await;
    }
}
