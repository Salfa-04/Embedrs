//!
//! # LED Task
//!

use crate::{hal, init_ticker};
use hal::{gpio, peripherals, time::khz, timer};

use gpio::OutputType;
use peripherals::{PH10, PH11, PH12, TIM5};
use timer::low_level::CountingMode::EdgeAlignedUp;
use timer::simple_pwm::{PwmPin, SimplePwm, SimplePwmChannels};

const RGB_CHANGE_HZ: u32 = 1000;
const RGB_FLOW_LENTH: usize = 7;

const RGB_FLOW_COLOR: [u32; RGB_FLOW_LENTH] = [
    0xFF0000FF, // blue
    0x0000FF00, // green(dark)
    0xFFFF0000, // red
    0x000000FF, // blue(dark)
    0xFF00FF00, // green
    0x00FF0000, // red(dark)
    0xFF0000FF, // blue
];

#[super::task]
pub async fn led_task(p: (TIM5, PH12, PH11, PH10)) -> ! {
    let mut t = init_ticker!(1000 / RGB_CHANGE_HZ as u64);

    let r = PwmPin::new_ch3(p.1, OutputType::PushPull);
    let g = PwmPin::new_ch2(p.2, OutputType::PushPull);
    let b = PwmPin::new_ch1(p.3, OutputType::PushPull);

    let SimplePwmChannels {
        ch3: mut r,
        ch2: mut g,
        ch1: mut b,
        ..
    } = SimplePwm::new(p.0, Some(b), Some(g), Some(r), None, khz(1), EdgeAlignedUp).split();

    (r.enable(), g.enable(), b.enable());

    loop {
        for i in 0..RGB_FLOW_LENTH - 1 {
            let mut alpha = ((RGB_FLOW_COLOR[i] & 0xFF000000) >> 24) as f32;
            let mut red = ((RGB_FLOW_COLOR[i] & 0x00FF0000) >> 16) as f32;
            let mut green = ((RGB_FLOW_COLOR[i] & 0x0000FF00) >> 8) as f32;
            let mut blue = ((RGB_FLOW_COLOR[i] & 0x000000FF) >> 0) as f32;

            let mut delta_alpha = ((RGB_FLOW_COLOR[i + 1] & 0xFF000000) >> 24) as f32 - alpha;
            let mut delta_red = ((RGB_FLOW_COLOR[i + 1] & 0x00FF0000) >> 16) as f32 - red;
            let mut delta_green = ((RGB_FLOW_COLOR[i + 1] & 0x0000FF00) >> 8) as f32 - green;
            let mut delta_blue = ((RGB_FLOW_COLOR[i + 1] & 0x000000FF) >> 0) as f32 - blue;

            delta_alpha /= RGB_CHANGE_HZ as f32;
            delta_red /= RGB_CHANGE_HZ as f32;
            delta_green /= RGB_CHANGE_HZ as f32;
            delta_blue /= RGB_CHANGE_HZ as f32;

            for _ in 0..RGB_CHANGE_HZ {
                alpha += delta_alpha;
                red += delta_red;
                green += delta_green;
                blue += delta_blue;

                r.set_duty_cycle_fraction((red * alpha) as u16, u16::MAX);
                g.set_duty_cycle_fraction((green * alpha) as u16, u16::MAX);
                b.set_duty_cycle_fraction((blue * alpha) as u16, u16::MAX);

                t.next().await;
            }
        }
    }
}
