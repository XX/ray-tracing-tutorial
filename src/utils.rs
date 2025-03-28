use std::fmt::Display;
use std::io::Write;
use std::time::{Duration, Instant};

pub struct Logger<T> {
    out: T,
}

impl<T: Write> Logger<T> {
    pub fn new(out: T) -> Self {
        Self { out }
    }

    pub fn ln(&mut self) -> &mut Self {
        writeln!(&mut self.out).expect("Failed to write");
        self
    }

    pub fn flush(&mut self) -> &mut Self {
        self.out.flush().expect("Failed to flush");
        self
    }

    pub fn msg(&mut self, msg: impl Display) -> &mut Self {
        write!(&mut self.out, "{}", msg).expect("Failed to write");
        self
    }

    pub fn elapsed(&mut self, timer: &Timer) -> &mut Self {
        self.msg(format!(
            "elapsed {:.6} seconds.",
            timer.elapsed().as_secs_f64()
        ));
        self
    }
}

pub struct Timer {
    instant: Instant,
    elapsed: Duration,
}

impl Timer {
    pub fn start() -> Timer {
        Self {
            instant: Instant::now(),
            elapsed: Duration::new(0, 0),
        }
    }

    pub fn stop(&mut self) {
        self.elapsed = self.instant.elapsed();
    }

    pub fn elapsed(&self) -> Duration {
        self.elapsed
    }
}
