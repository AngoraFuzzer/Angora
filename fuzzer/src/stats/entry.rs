use super::*;
use colored::*;
use serde_derive::Serialize;

#[derive(
    Default, Clone, Copy, Eq, PartialEq, Add, AddAssign, From, Into, Ord, PartialOrd, Serialize,
)]
pub struct Counter(pub usize);

impl Counter {
    pub fn count(&mut self) {
        self.0 += 1;
    }
}

impl fmt::Display for Counter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>7}", format_count(self.0).bold())
    }
}

#[derive(Clone, Copy, Constructor, Serialize)]
pub struct Average(pub f32, usize);

impl Default for Average {
    fn default() -> Self {
        Average(0.0, 0)
    }
}

impl fmt::Display for Average {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:>7.2}", self.0).yellow())
    }
}

impl Average {
    pub fn update(&mut self, v: f32) {
        if self.1 == 0 {
            self.0 = v;
        } else {
            let p = 1.0 / (self.1 + 1) as f32;
            self.0 = self.0 * (1.0 - p) + v * p;
        }
        self.1 += 1;
    }

    pub fn get(&self) -> f32 {
        self.0
    }

    pub fn synthesize(&mut self, other: &Self) {
        let n = self.1 + other.1;
        if n > 0 {
            let p = self.1 as f32 / n as f32;
            self.0 = self.0 * p + other.0 * (1.0 - p);
            self.1 = n;
        }
    }

    pub fn get_ratio(&self, v: f32) -> usize {
        if self.0 == 0.0 {
            1
        } else {
            (v / self.0) as usize
        }
    }
}

#[derive(Clone, Copy, Serialize)]
pub struct SyncAverage {
    local: Average,
    global: Average,
}

impl Default for SyncAverage {
    fn default() -> Self {
        Self {
            local: Default::default(),
            global: Default::default(),
        }
    }
}

impl SyncAverage {
    pub fn update(&mut self, v: f32) {
        self.local.update(v);
    }

    pub fn get(&self) -> f32 {
        self.global.get()
    }

    pub fn sync(&mut self, other: &mut Average) {
        other.synthesize(&self.local);
        self.local = Default::default();
        self.global = other.clone();
    }

    pub fn get_ratio(&self, v: f32) -> usize {
        self.global.get_ratio(v)
    }
}

#[derive(Default, Clone, Copy, Add, AddAssign, From, Into, Serialize)]
pub struct TimeDuration(pub time::Duration);

impl fmt::Display for TimeDuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format_time(self.0).cyan())
    }
}

#[derive(Clone, Copy)]
pub struct TimeIns(pub time::Instant);

impl Into<TimeDuration> for TimeIns {
    fn into(self) -> TimeDuration {
        self.0.elapsed().into()
    }
}

impl Default for TimeIns {
    fn default() -> Self {
        TimeIns(time::Instant::now())
    }
}

impl fmt::Display for TimeIns {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format_time(self.0.elapsed()).cyan())
    }
}

impl serde::ser::Serialize for TimeIns {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_u64(self.0.elapsed().as_secs())
    }
}
