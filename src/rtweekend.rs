use rand::{thread_rng, Rng};

const PI: f64 = std::f64::consts::PI;

pub fn degrees_to_radius(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_f64() -> f64 {
    let mut rng = thread_rng();
    // [0, 1)
    rng.gen()
}

pub fn random_f64_in_range(min: f64, max: f64) -> f64 {
    let mut rng = thread_rng();
    // [min, max)
    rng.gen_range(min..max)
}
