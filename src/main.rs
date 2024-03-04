use color::make_color;
use vec3::make_point;

use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

mod color;
mod hit_record;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().length_squared();
    let half_b = vec3::dot(oc, r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(r: ray::Ray) -> color::Color {
    let unit_direction = vec3::unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);

    let t = hit_sphere(make_point(0.0, 0.0, -1.0), 0.5, &r);
    if t > 0.0 {
        let n = vec3::unit_vector(r.at(t) - vec3::Vec3(0.0, 0.0, -1.0));
        return 0.5 * make_color(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }
    (1.0 - a) * make_color(1.0, 1.0, 1.0) + a * make_color(0.5, 0.7, 1.0)
}

fn main() {
    // Image Configuration

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;

    let mut image_height: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    image_height = if image_height < 1 { 1 } else { image_height };

    // Camera Configuration
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (IMAGE_WIDTH as f64 / image_height as f64);
    let camera_center = vec3::make_point(0.0, 0.0, 0.0);

    let viewport_u = Vec3(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left =
        camera_center - Vec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    print!("P3\n{IMAGE_WIDTH} {image_height}\n255\n");
    let mut j = 0;
    while j < image_height {
        let lines_remaining = image_height - j;
        eprint!("Scanlines remaining: {lines_remaining}\r");
        let mut i = 0;
        while i < IMAGE_WIDTH {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray {
                orgin: camera_center,
                direction: ray_direction,
            };
            let pixel_color = ray_color(r);
            color::write_color(pixel_color);
            i += 1;
        }
        j += 1;
    }
}
