use std::time::Duration;

pub struct Timer {
    duration: Duration,
    remaining: Duration,
    started: bool,
    looping: bool,
    finished: bool,
}

impl Timer {
    pub fn new(duration: Duration, looping: bool) -> Self {
        Self {
            duration,
            remaining: duration,
            started: false,
            looping,
            finished: false,
        }
    }

    pub fn update(&mut self, time_delta: Duration) {
        if self.finished() && self.looping {
            self.reset()
        }

        if self.remaining < time_delta {
            self.remaining = Duration::from_secs(0);
            self.finished = true
        } else if self.started {
            self.remaining -= time_delta
        }
    }

    pub fn reset(&mut self) {
        self.remaining = self.duration;
        self.finished = false;
        self.started = self.looping
    }

    pub fn start(&mut self) {
        self.started = true
    }

    pub fn started(&self) -> bool {
        self.started
    }

    pub fn stop(&mut self) {
        self.started = false
    }

    pub fn finished(&self) -> bool {
        self.finished
    }

    pub fn elapsed(&self) -> Duration {
        self.duration - self.remaining
    }

    pub fn remaining(&self) -> Duration {
        self.remaining
    }
}
