use crate::interval;
use crate::rtweekend::random_f64;
use crate::{color::Color, hittable::Hittable, ray::Ray};
use std::f64::INFINITY;

use crate::color::{make_color, write_color};
use crate::hit_record::HitRecord;
use crate::vec3::{random_unit, Vec3};
use crate::vec3::{make_point, unit_vector};
use crate::vec3::{random_on_hemisphere, Point3};
// TODO: Clean up the imports

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            image_height: 100,
            center: make_point(0.0, 0.0, 0.0),
            pixel00_loc: make_point(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3(0.0, 0.0, 0.0),
            samples_per_pixel: 10,
            max_depth: 10,
        }
    }
}

impl Camera {
    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.center = make_point(0.0, 0.0, 0.0);

        // Viewport Dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width =
            viewport_height * (self.image_width as f64) / (self.image_height as f64);

        // Vectors to help navigate the viewport during the render
        let viewport_u = Vec3(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3(0.0, -viewport_height, 0.0);

        // Delta vectors for pixel-to-pixel distances
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Find the upper left pixel
        let viewport_upper_left =
            self.center - Vec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        print!("P3\n{0} {1}\n255\n", self.image_width, self.image_height);
        let mut j = 0;
        while j < self.image_height {
            let lines_remaining = self.image_height - j;
            eprint!("Scanlines remaining: {lines_remaining}\r");
            let mut i = 0;
            while i < self.image_width {
                let mut pixel_color = make_color(0.0, 0.0, 0.0);
                let mut sample = 0;
                while sample < self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(r, self.max_depth, world);
                    sample += 1;
                }
                write_color(pixel_color, self.samples_per_pixel);
                i += 1;
            }
            j += 1;
        }
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center =
            self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();
        let ray_direction = pixel_sample - self.center;
        Ray {
            orgin: self.center,
            direction: ray_direction,
        }
    }

    fn ray_color(&self, r: Ray, depth: i32, world: &dyn Hittable) -> Color {
        let rec = &mut HitRecord::default();
        if depth <= 0 {
            return make_color(0.0, 0.0, 0.0);
        }
        if world.hit(&r, interval::new(0.001, INFINITY), rec) {
            let direction = rec.normal + random_unit();
            return 0.5
                * self.ray_color(
                    Ray {
                        orgin: rec.p,
                        direction,
                    },
                    depth - 1,
                    world,
                );
        }

        let unit_direction = unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - a) * make_color(1.0, 1.0, 1.0) + a * make_color(0.5, 0.7, 1.0);
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 * random_f64();
        let py = -0.5 + random_f64();
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }
}
