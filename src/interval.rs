#[derive(Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: f64::INFINITY,
            max: -f64::INFINITY,
        }
    }
}

impl Interval {
    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn size(self) -> f64 {
        self.max - self.min
    }

    pub fn contains(self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(self, x: f64) -> f64 {
        match x {
            x if x < self.min => self.min,
            x if x > self.max => self.max,
            _ => x,
        }
    }

    pub const EMPTY: Interval = Interval::new(f64::INFINITY, -f64::INFINITY);
    pub const UNIVERSE: Interval = Interval::new(-f64::INFINITY, f64::INFINITY);
}
