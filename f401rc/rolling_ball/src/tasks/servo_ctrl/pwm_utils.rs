use crate::hal;

use gpio::OutputType;
use hal::timer::simple_pwm::SimplePwmChannel;
use hal::{gpio, peripherals, time::hz, timer};
use peripherals::{PA6, PA7, TIM3};
use timer::low_level::CountingMode::EdgeAlignedUp;
use timer::simple_pwm::{PwmPin, SimplePwm};

pub struct ServoPwm {
    duty_cycle: Option<(f32, f32)>,
    duty_cycle_step: (f32, f32),

    duty_step: f32,
}

impl ServoPwm {
    pub const fn new(duty_step: f32) -> ServoPwm {
        let s = unsafe { core::mem::zeroed() };
        Self { duty_step, ..s }
    }

    pub fn set(&mut self, set: (f32, f32)) {
        if self.duty_cycle.is_none() {
            self.duty_cycle = Some(set);
            self.duty_cycle_step = set;
        } else {
            self.duty_cycle = Some(set);
        }
    }

    pub fn finished(&self) -> bool {
        if let Some(duty_cycle) = self.duty_cycle {
            duty_cycle.0 == self.duty_cycle_step.0 && duty_cycle.1 == self.duty_cycle_step.1
        } else {
            true
        }
    }

    pub fn calc(&mut self) -> (f32, f32) {
        if let Some(duty_cycle) = self.duty_cycle {
            if duty_cycle.0 > self.duty_cycle_step.0 {
                self.duty_cycle_step.0 += self.duty_step;
                if duty_cycle.0 <= self.duty_cycle_step.0 {
                    self.duty_cycle_step.0 = duty_cycle.0;
                }
            } else if duty_cycle.0 < self.duty_cycle_step.0 {
                self.duty_cycle_step.0 -= self.duty_step;
                if duty_cycle.0 >= self.duty_cycle_step.0 {
                    self.duty_cycle_step.0 = duty_cycle.0;
                }
            }

            if duty_cycle.1 > self.duty_cycle_step.1 {
                self.duty_cycle_step.1 += self.duty_step;
                if duty_cycle.1 <= self.duty_cycle_step.1 {
                    self.duty_cycle_step.1 = duty_cycle.1;
                }
            } else if duty_cycle.1 < self.duty_cycle_step.1 {
                self.duty_cycle_step.1 -= self.duty_step;
                if duty_cycle.1 >= self.duty_cycle_step.1 {
                    self.duty_cycle_step.1 = duty_cycle.1;
                }
            }
        }

        (self.duty_cycle_step.0, self.duty_cycle_step.1)
    }
}

pub async fn pwm_init(
    p: (TIM3, PA6, PA7),
) -> (
    SimplePwmChannel<'static, TIM3>,
    SimplePwmChannel<'static, TIM3>,
    u16,
) {
    let pin_x = PwmPin::new_ch1(p.1, OutputType::PushPull);
    let pin_y = PwmPin::new_ch2(p.2, OutputType::PushPull);

    let pwm = SimplePwm::new(
        p.0,
        Some(pin_x),
        Some(pin_y),
        None,
        None,
        hz(50),
        EdgeAlignedUp,
    );

    let max_duty_cycle = pwm.max_duty_cycle();
    let channels = pwm.split();
    let mut ch_x = channels.ch1;
    let mut ch_y = channels.ch2;

    (ch_x.enable(), ch_y.enable());

    (ch_x, ch_y, max_duty_cycle)
}
