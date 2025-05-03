use super::consts::POSI;
use super::{POSI_NUM, impls::PidTd};

pub async fn pid_init() -> (PidTd, PidTd) {
    let position = { *POSI_NUM.lock().await };

    let mut pid_v = PidTd::new(
        (0.0, 0.0), // Set
        (14.0, 14.0),
    );

    let mut pid_p = PidTd::new(
        POSI[position], // Set
        (320.0, 320.0),
    );

    // Speed
    // pid_v.0.p(0.7, 4.0).i(0.005, 1.0).d(9.0, 12.0);
    // pid_v.1.p(0.7, 4.0).i(0.005, 1.0).d(9.0, 12.0);

    pid_v.0.p(0.67, 4.0).i(0.011, 1.0).d(8.3, 12.0);
    pid_v.1.p(0.67, 4.0).i(0.011, 1.0).d(8.3, 12.0);

    // Point
    pid_p.0.p(0.2, 320.0).i(0.0, 0.0).d(0.0, 0.0);
    pid_p.1.p(0.2, 320.0).i(0.0, 0.0).d(0.0, 0.0);

    (pid_v, pid_p)
}
