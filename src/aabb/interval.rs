pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn new_adjusted(a: f64, b: f64) -> Self {
        if a < b {
            Self::new(a, b)
        } else {
            Self::new(b, a)
        }
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn empty(&self) -> bool {
        self.min > self.max
    }

    pub fn size(&self) -> f64 {
        if self.empty() {
            0.0
        } else {
            self.max - self.min
        }
    }

    pub fn expand(&self, delta: f64) -> Self {
        Self::new(self.min - delta / 2.0, self.max + delta / 2.0)
    }

    pub fn overlap(&self, x: &Interval) -> Self {
        Self::new(self.min.max(x.min), self.max.min(x.max))
    }

    pub fn overlaps(&mut self, x: &Interval) {
        if x.min > self.min {
            self.min = x.min;
        }
        if x.max < self.max {
            self.max = x.max;
        }
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self::new(f64::INFINITY, f64::NEG_INFINITY)
    }
}
