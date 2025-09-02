use std::time::{Duration, Instant};

pub struct TimeInfo {
    target_fps: u64,
    current_on_target: u64,
    current: u64,
    second_start: Instant,
    prev_time: Instant,
    delta_time: Duration,
    average: Duration,
    fps: u64,
}

impl TimeInfo {
    pub fn new(fps: u64) -> Self {
        Self {
            target_fps: fps,
            prev_time: Instant::now(),
            current_on_target: 0,
            current: 0,
            delta_time: Duration::new(0, 0),
            average: Duration::new(0, 0),
            second_start: Instant::now(),
            fps,
        }
    }
    pub fn start(&mut self) {
        self.prev_time = Instant::now();
        self.second_start = Instant::now();
    }

    pub fn set_fps(&mut self, fps: u64) {
        self.target_fps = fps;
        unimplemented!("NO RESET OF TIME INFO YET")
    }

    fn increment_frame(&mut self) {
        if self.current_on_target + 1 < self.target_fps {
            self.current_on_target += 1;
        } else {
            self.current_on_target = 0;
        }

        self.current += 1;
    }

    pub fn update(&mut self) -> bool {
        let mut scnd = false;
        self.delta_time = self.prev_time.elapsed();
        self.prev_time = Instant::now();
        self.increment_frame();
        if self.second_start.elapsed() >= Duration::from_secs(1) {
            self.average =
                Duration::from_millis(((1.0 / self.current_on_target as f64) * 1000.0) as u64);
            scnd = true;
            self.fps = self.current;
            self.current = 0;
            self.second_start = Instant::now();
        }
        return scnd;
    }

    pub fn get_average_delta_ms(&self) -> u128 {
        self.average.as_millis()
    }

    pub fn get_average_fps(&self) -> u64 {
        self.fps
    }

    pub fn delta_time_ms(&self) -> u128 {
        self.delta_time.as_millis()
    }

    pub fn delta_time_s(&self) -> f64 {
        self.delta_time.as_millis() as f64 / 1000.0
    }
}
