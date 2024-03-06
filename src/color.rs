use crate::{interval, vec3::Vec3};
pub type Color = Vec3;

pub(crate) fn make_color(r: f64, g: f64, b: f64) -> Color {
    Vec3(r, g, b)
}

pub fn write_color(pixel_color: Color, samples_per_pixel: i32) -> () {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale = 1.0 / (samples_per_pixel as f64);
    r = linear_to_gamma_2(r * scale);
    g = linear_to_gamma_2(g * scale);
    b = linear_to_gamma_2(b * scale);

    let intensity = interval::new(0.000, 0.999);

    print!(
        "{0} {1} {2}\n",
        (256.0 * intensity.clamp(r)) as i32,
        (256.0 * intensity.clamp(g)) as i32,
        (256.0 * intensity.clamp(b)) as i32,
    )
}

pub fn linear_to_gamma_2(linear_component: f64) -> f64 {
    linear_component.sqrt()
}
