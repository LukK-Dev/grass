use std::time::{Duration, Instant};

pub struct Timing {
    start_instant: Instant,
    last_frame_instant: Instant,
}

impl Timing {
    pub fn new() -> Self {
        Self {
            start_instant: Instant::now(),
            last_frame_instant: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        self.last_frame_instant = Instant::now()
    }

    pub fn time_since_start(&self) -> Duration {
        Instant::now() - self.start_instant
    }

    pub fn time_delta(&self) -> Duration {
        Instant::now() - self.last_frame_instant
    }

    pub fn fps(&self) -> u32 {
        (1.0 / self.time_delta().as_secs_f32()) as u32
    }
}
