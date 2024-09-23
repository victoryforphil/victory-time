use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Timecode {
    pub secs: u64,
    pub nanos: u32,
}

impl Default for Timecode {
    fn default() -> Self {
        Timecode { secs: 0, nanos: 0 }
    }
}

impl Timecode {
    pub fn new_secs(secs: f64) -> Timecode {
        let secs = secs as f64;
        let nanos = (secs.fract() * 1_000_000_000.0) as u32;
        Timecode {
            secs: secs as u64,
            nanos,
        }
    }

    pub fn new(secs: u64, nanos: u32) -> Timecode {
        Timecode { secs, nanos }
    }

    pub fn zero() -> Timecode {
        Timecode { secs: 0, nanos: 0 }
    }

    pub fn new_hz(hz: f64) -> Timecode {
        let secs = 1.0 / hz;
        Timecode::new_secs(secs)
    }

    pub fn new_ms(ms: f64) -> Timecode {
        let secs = ms / 1000.0;
        Timecode::new_secs(secs)
    }

    pub fn new_us(us: f64) -> Timecode {
        let secs = us / 1_000_000.0;
        Timecode::new_secs(secs)
    }

    pub fn secs(&self) -> f64 {
        self.secs as f64 + (self.nanos as f64 / 1_000_000_000.0)
    }

    pub fn ms(&self) -> f64 {
        self.secs as f64 * 1000.0 + (self.nanos as f64 / 1_000_000.0)
    }

    pub fn us(&self) -> f64 {
        self.secs as f64 * 1_000_000.0 + (self.nanos as f64 / 1_000.0)
    }
}
