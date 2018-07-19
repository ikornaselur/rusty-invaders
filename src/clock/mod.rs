use std::time::{Duration, Instant};

pub struct Clock {
    last_time: Instant,
}

impl Default for Clock {
    fn default() -> Clock {
        Clock::new()
    }
}

impl Clock {
    pub fn new() -> Clock {
        Clock {
            last_time: Instant::now(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.last_time.elapsed()
    }

    pub fn reset_last_time(&mut self) -> () {
        self.last_time = Instant::now();
    }
}
