use camera::Camera;
use color::make_color;
use material::{make_lambertian, make_metal};
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
    let material_ground = make_lambertian(make_color(0.8, 0.8, 0.0));
    let material_center = make_lambertian(make_color(0.7, 0.3, 0.3));
    let material_left = make_metal(make_color(0.8, 0.8, 0.8));
    let material_right = make_metal(make_color(0.8, 0.6, 0.2));

    world.add(Box::new(make_sphere(
        make_point(0.0, -100.5, -1.0),
        100.0,
        Box::new(material_ground),
    )));
    world.add(Box::new(make_sphere(
        make_point(0.0, 0.0, -1.0),
        0.5,
        Box::new(material_center),
    )));
    world.add(Box::new(make_sphere(
        make_point(-1.0, 0.0, -1.0),
        0.5,
        Box::new(material_left),
    )));
    world.add(Box::new(make_sphere(
        make_point(1.0, 0.0, -1.0),
        0.5,
        Box::new(material_right),
    )));

    let mut camera = Camera::default();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.render(&world);
}
