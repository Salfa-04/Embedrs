use pid::{Number, Pid};

pub struct PidTd<T: Number>(pub Pid<T>, pub Pid<T>);

pub trait ClearPID {
    fn clear(&mut self);
}

pub trait ServoLimit {
    const LIMIT: (f32, f32);
    fn limit(&self) -> (f32, f32);
}

impl<T: Number> PidTd<T> {
    pub fn new<NUM: Into<T>>(set: (NUM, NUM), limit: (NUM, NUM)) -> PidTd<T> {
        Self(
            Pid::new(set.0, limit.0), // x
            Pid::new(set.1, limit.1), // y
        )
    }

    pub fn setpoint<NUM: Into<T>>(&mut self, set: (NUM, NUM)) {
        self.0.setpoint(set.0);
        self.1.setpoint(set.1);
    }

    pub fn calculate<NUM: Into<T>>(&mut self, feadback: (NUM, NUM)) -> (T, T) {
        (
            self.0.next_control_output(feadback.0.into()).output,
            self.1.next_control_output(feadback.1.into()).output,
        )
    }
}

impl<T: Number> ClearPID for PidTd<T> {
    fn clear(&mut self) {
        (self.0.reset_integral_term(), self.1.reset_integral_term());
        let _ = self.0.next_control_output(self.0.setpoint);
        let _ = self.1.next_control_output(self.1.setpoint);
    }
}

impl ServoLimit for (f32, f32) {
    const LIMIT: (f32, f32) = (15.0, 15.0);

    fn limit(&self) -> (f32, f32) {
        (
            self.0.clamp(-Self::LIMIT.0, Self::LIMIT.0),
            self.1.clamp(-Self::LIMIT.1, Self::LIMIT.1),
        )
    }
}
