use std::time::{Duration, SystemTime};

pub struct Clock {
    last_time: SystemTime,
}

impl Clock {
    pub fn new() -> Clock {
        Clock {
            last_time: SystemTime::now(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        SystemTime::now()
            .duration_since(self.last_time)
            .expect("Negative time elapsed")
    }

    pub fn reset_last_time(&mut self) -> () {
        self.last_time = SystemTime::now();
    }
}
