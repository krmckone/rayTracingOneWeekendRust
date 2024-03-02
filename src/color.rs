use crate::vec3::Vec3;
pub type Color = Vec3;

pub(crate) fn make_color(r: f64, g: f64, b: f64) -> Color {
    Vec3(r, g, b)
}

pub fn write_color(pixel_color: Color) -> () {
    let r = (255.999 * pixel_color.x()) as i32;
    let g = (255.999 * pixel_color.y()) as i32;
    let b = (255.999 * pixel_color.z()) as i32;

    print!("{r} {g} {b}\n")
}
