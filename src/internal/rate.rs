use std::time::{Duration, Instant};

//const SEC_IN_MS: usize = 1000;
const DEFAULT_FPS: usize = 60;

fn fps_to_rate(fps: usize) -> Option<Duration> {
    match fps {
        0 => None,
        f => Some(Duration::from_secs_f32(1. / f as f32)),
    }
}

pub struct Fps {
    target_fps: usize,
    target_rate: Option<Duration>,
    prev_time: Instant,
}

impl Fps {
    pub fn new() -> Self {
        Self::with_fps(DEFAULT_FPS)
    }
    pub fn with_fps(fps: usize) -> Self {
        let target_rate = fps_to_rate(fps);
        Self {
            target_fps: fps,
            target_rate,
            prev_time: Instant::now(),
        }
    }

    pub fn get_target_fps(&self) -> usize {
        self.target_fps
    }

    pub fn set_target_fps(&mut self, fps: usize) {
        let target_rate = fps_to_rate(fps);
        self.target_rate = target_rate;
        self.target_fps = fps;
    }

    pub fn update(&mut self) {
        if let Some(tr) = self.target_rate {
            let d = self.prev_time.elapsed();

            if d < tr {
                let delay = tr - d;
                std::thread::sleep(delay);
            }
            self.prev_time = Instant::now();
        }
    }
}
