use std::{
    fmt::Display,
    io::Write,
    time::{Duration, Instant},
};

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

    pub fn progress_line(&mut self, n: impl Display) -> &mut Self {
        write!(&mut self.out, "\rScanlines remaining: {n}  ").expect("Failed to write");
        self
    }

    pub fn done(&mut self) -> &mut Self {
        write!(&mut self.out, "\rDone.                    ").expect("Failed to write");
        self
    }

    pub fn elapsed(&mut self, timer: &Timer) -> &mut Self {
        write!(
            &mut self.out,
            "\rElapsed {:.6} seconds.",
            timer.elapsed().as_secs_f64()
        )
        .expect("Failed to write");
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
