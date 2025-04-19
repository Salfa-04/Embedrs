use super::consts::POSI;
use super::{POSI_NUM, impls::PidTd};

pub async fn pid_init() -> (PidTd<f32>, PidTd<f32>) {
    let position = { *POSI_NUM.lock().await };

    let mut pid_v = PidTd::<f32>::new(
        (0.0, 0.0), // Set
        (14.0, 14.0),
    );

    let mut pid_p = PidTd::<f32>::new(
        POSI[position], // Set
        (320.0, 320.0),
    );

    // Speed
    // pid_v.0.p(0.7, 4.0).i(0.005, 1.0).d(9.0, 12.0);
    // pid_v.1.p(0.7, 4.0).i(0.005, 1.0).d(9.0, 12.0);

    pid_v.0.p(0.75, 4.0).i(0.011, 1.0).d(7.5, 12.0);
    pid_v.1.p(0.75, 4.0).i(0.011, 1.0).d(7.5, 12.0);

    // Point
    pid_p.0.p(0.2, 320.0).i(0.0, 0.0).d(0.0, 0.0);
    pid_p.1.p(0.2, 320.0).i(0.0, 0.0).d(0.0, 0.0);

    (pid_v, pid_p)
}
