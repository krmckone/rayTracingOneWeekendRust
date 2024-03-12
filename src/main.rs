use std::f64::consts::PI;

use camera::Camera;
use color::make_color;
use material::{make_dielectric, make_lambertian, make_metal};
use sphere::make_sphere;

use crate::{hittable_list::HittableList, vec3::make_point};

mod camera;
mod color;
mod hit_record;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

fn main() {
    // World Setup
    let mut world = HittableList {
        objects: Vec::new(),
    };

    let r = (PI / 4.0).cos();
    let material_left = make_lambertian(make_color(0.0, 0.0, 1.0));
    let material_right = make_lambertian(make_color(1.0, 0.0, 0.0));

    world.add(Box::new(make_sphere(
        make_point(-r, -0.0, -1.0),
        r,
        Box::new(material_left),
    )));
    world.add(Box::new(make_sphere(
        make_point(r, 0.0, -1.0),
        r,
        Box::new(material_right),
    )));

    let mut camera = Camera::default();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.vfov = 90;

    camera.render(&world);
}
