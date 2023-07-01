use std::time::{Duration, Instant};

pub struct Timing {
    start_instant: Instant,
    last_frame_instant: Instant,
    time_delta: Duration,
}

impl Timing {
    pub fn new() -> Self {
        Self {
            start_instant: Instant::now(),
            last_frame_instant: Instant::now(),
            time_delta: Duration::from_secs(0),
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.time_delta = now - self.last_frame_instant;
        self.last_frame_instant = now;
    }

    pub fn time_since_start(&self) -> Duration {
        Instant::now() - self.start_instant
    }

    pub fn time_delta(&self) -> Duration {
        self.time_delta
    }

    pub fn fps(&self) -> u32 {
        (1.0 / self.time_delta.as_secs_f32()) as u32
    }
}
