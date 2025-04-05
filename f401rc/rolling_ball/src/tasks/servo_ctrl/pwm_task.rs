//!
//! # Pwm Task
//!

use crate::{hal, init_ticker};

use super::pwm_utils::{ServoPwm, pwm_init};
use embassy_sync::{self as sync, once_lock, signal};
use hal::peripherals::{PA6, PA7, TIM3};
use sync::blocking_mutex::raw::ThreadModeRawMutex as RM;
use {once_lock::OnceLock, signal::Signal};

static MAX_DUTY_CYCLE: OnceLock<u16> = OnceLock::new();
static DUTY_CYCLE: Signal<RM, (f32, f32)> = Signal::new();

/// PWM Duty Cycle Set
/// x, y: from -135 to 135
pub async fn set_servo(angle: (f32, f32)) {
    let (x, y) = angle;

    assert!(x >= -135.0 && x <= 135.0);
    assert!(y >= -135.0 && y <= 135.0);

    let max = *MAX_DUTY_CYCLE.get().await;
    // duty_cycle_percent = (x / 135° + 1.5) / 20ms
    //          x = -135° to 135°
    // set = duty_cycle_percent * duty_cycle_max
    DUTY_CYCLE.signal((
        (x as f32 + 202.5) * max as f32 / 2700f32,
        (y as f32 + 202.5) * max as f32 / 2700f32,
    ));
}

#[super::task]
pub async fn pwm_task(p: (TIM3, PA6, PA7)) -> ! {
    let mut t = init_ticker!(20);

    let (mut ch_x, mut ch_y, max_duty_cycle) = pwm_init(p).await;

    MAX_DUTY_CYCLE.init(max_duty_cycle).unwrap();

    // Duty Cycle Step Calc:
    // Servo Speed: 0.16s/60°
    // ~ => 160ms/60° => 20ms/7.5°
    // ~ => ∆7.5°  ~ ∆7.5/2700 * max ms
    // ~ => max / 360 ms
    let duty_step = *MAX_DUTY_CYCLE.get().await as f32 / 360f32;
    let mut servo = ServoPwm::new(duty_step);

    loop {
        if DUTY_CYCLE.signaled() {
            if let Some((x, y)) = DUTY_CYCLE.try_take() {
                servo.set((x, y));
                // defmt::info!("Duty Cycle: {}", (x, y));
            }
        }

        if servo.finished() {
            let duty_cycle = DUTY_CYCLE.wait().await;
            servo.set(duty_cycle);
            // defmt::info!("Duty Cycle: {}", duty_cycle);
        }

        // Update Step Duty Cycle
        let (ox, oy) = servo.calc();
        // defmt::info!("Duty Cycle: {}", (ox, oy));

        ch_x.set_duty_cycle(ox as u16);
        ch_y.set_duty_cycle(oy as u16);

        t.next().await;
    }
}
