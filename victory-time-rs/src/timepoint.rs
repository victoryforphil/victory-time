use std::{ops::Add, sync::Arc, time::Instant};

use crate::timespan::Timespan;

use super::Timecode;
use serde::{Deserialize, Serialize};
pub type TimepointHandle = Arc<Timepoint>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Timepoint {
    pub time: Timecode,
}
impl Default for Timepoint {
    fn default() -> Self {
        Timepoint {
            time: Timecode::zero(),
        }
    }
}

impl Timepoint {
    pub fn new(time: Timecode) -> Timepoint {
        Timepoint { time }
    }
    pub fn new_secs(secs: f64) -> Timepoint {
        Self::new(Timecode::new_secs(secs))
    }

    pub fn new_ms(ms: f64) -> Timepoint {
        Self::new(Timecode::new_ms(ms))
    }

    pub fn new_us(us: f64) -> Timepoint {
        Self::new(Timecode::new_us(us))
    }

    pub fn now() -> Timepoint {
        let now = Instant::now();
        now.into()
    }

    pub fn handle(&self) -> TimepointHandle {
        Arc::new(self.clone())
    }

    pub fn zero() -> Timepoint {
        Timepoint {
            time: Timecode::zero(),
        }
    }

    pub fn secs(&self) -> f64 {
        self.time.secs()
    }

    pub fn ms(&self) -> f64 {
        self.time.ms()
    }
}

impl Add<Timespan> for Timepoint {
    type Output = Timepoint;

    fn add(self, rhs: Timespan) -> Self::Output {
        let duration_time = rhs.time;
        let new_time = Timecode {
            secs: self.time.secs + duration_time.secs,
            nanos: self.time.nanos + duration_time.nanos,
        };
        Timepoint::new(new_time)
    }
}

impl From<Instant> for Timepoint {
    fn from(instant: Instant) -> Self {
        let duration = instant.elapsed();
        let secs = duration.as_secs();
        let nanos = duration.subsec_nanos();
        Timepoint {
            time: Timecode::new(secs, nanos),
        }
    }
}

impl From<Arc<Timepoint>> for Timepoint {
    fn from(handle: Arc<Timepoint>) -> Self {
        handle.as_ref().clone()
    }
}

impl From<Timecode> for Timepoint {
    fn from(time: Timecode) -> Self {
        Timepoint { time }
    }
}
