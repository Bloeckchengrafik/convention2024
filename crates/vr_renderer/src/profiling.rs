use std::time::Instant;

pub struct Profiler {
    pub start: Option<Instant>,
    pub lap: Option<Instant>,
}

impl Profiler {
    pub fn new(enable: bool) -> Profiler {
        Profiler {
            start: if enable { Some(Instant::now()) } else { None },
            lap: if enable { Some(Instant::now()) } else { None },
        }
    }

    pub fn print_elapsed(&mut self, title: &str) {
        if let Some(start) = self.start {
            info!("[{}] {:?} (Lap: {:?})", title, start.elapsed(), self.lap.unwrap().elapsed());
            self.lap = Some(Instant::now());
        }
    }
}