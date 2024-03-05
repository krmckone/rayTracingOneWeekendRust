use camera::Camera;

use crate::{hittable_list::HittableList, sphere::Sphere, vec3::make_point};

mod camera;
mod color;
mod hit_record;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

fn main() {
    // World Setup
    let mut world = HittableList {
        objects: Vec::new(),
    };
    world.add(Box::new(Sphere {
        center: make_point(0.0, 0.0, -1.0),
        radius: 0.5,
    }));
    world.add(Box::new(Sphere {
        center: make_point(0.0, -100.5, -1.0),
        radius: 100.0,
    }));

    let mut camera = Camera::default();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;

    camera.render(&world);
}
