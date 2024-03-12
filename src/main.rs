use std::f64::consts::PI;

use camera::Camera;
use color::make_color;
use material::{make_dielectric, make_lambertian, make_metal};
use sphere::make_sphere;
use vec3::Vec3;

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
    let material_center = make_lambertian(make_color(0.1, 0.2, 0.5));
    let material_left = make_dielectric(1.5);
    let material_left_bubble = make_dielectric(1.5);
    let material_right = make_metal(make_color(0.8, 0.6, 0.2), 0.0);

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
        make_point(-1.0, 0.0, -1.0),
        -0.4,
        Box::new(material_left_bubble),
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

    camera.vfov = 20;
    camera.lookfrom = make_point(-2.0, 2.0, 1.0);
    camera.lookat = make_point(0.0, 0.0, -1.0);
    camera.vup = Vec3(0.0, 1.0, 0.0);

    camera.render(&world);
}
