use std::f64::INFINITY;

const EMPTY: Interval = Interval {
    min: INFINITY,
    max: -INFINITY,
};

const UNIVERSE: Interval = Interval {
    min: -INFINITY,
    max: INFINITY,
};

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: INFINITY,
            max: -INFINITY,
        }
    }
}

impl Interval {
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
}

pub fn new(min: f64, max: f64) -> Interval {
    Interval { min, max }
}
