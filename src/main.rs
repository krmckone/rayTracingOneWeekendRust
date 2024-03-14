use camera::Camera;
use color::make_color;
use material::{make_dielectric, make_lambertian, make_metal, Material};
use rtweekend::{random_f64, random_f64_in_range};
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

    let ground_material = make_lambertian(make_color(0.5, 0.5, 0.5));
    world.add(Box::new(make_sphere(
        make_point(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(ground_material),
    )));
    let mut a = -11.0;
    while a < 11.0 {
        let mut b = -11.0;
        while b < 11.0 {
            let choose_mat = random_f64();
            let center = make_point(a + 0.9 * random_f64(), 0.2, b + 0.9 * random_f64());

            if (center - make_point(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Box<dyn Material> = if choose_mat < 0.8 {
                    let albedo = vec3::random() * vec3::random();
                    Box::new(make_lambertian(albedo))
                } else if choose_mat < 0.95 {
                    let albedo = vec3::random_in_range(0.5, 1.0);
                    let fuzz = random_f64_in_range(0.0, 0.5);
                    Box::new(make_metal(albedo, fuzz))
                } else {
                    Box::new(make_dielectric(1.5))
                };
                world.add(Box::new(make_sphere(center, 0.2, sphere_material)));
            }
            b += 1.0;
        }
        a += 1.0;
    }

    let material_1 = make_dielectric(1.5);
    world.add(Box::new(make_sphere(
        make_point(0.0, 1.0, 0.0),
        1.0,
        Box::new(material_1),
    )));

    let material_2 = make_lambertian(make_color(0.4, 0.2, 0.1));
    world.add(Box::new(make_sphere(
        make_point(-4.0, 1.0, 0.0),
        1.0,
        Box::new(material_2),
    )));

    let material_3 = make_metal(make_color(0.7, 0.6, 0.5), 0.0);
    world.add(Box::new(make_sphere(
        make_point(4.0, 1.0, 0.0),
        1.0,
        Box::new(material_3),
    )));

    let mut camera = Camera::default();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1200;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.vfov = 20;
    camera.lookfrom = make_point(13.0, 2.0, 3.0);
    camera.lookat = make_point(0.0, 0.0, 0.0);
    camera.vup = Vec3(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;

    camera.render(&world);
}
