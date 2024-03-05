const PI: f64 = std::f64::consts::PI;

pub fn degrees_to_radius(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
