use crate::interval;
use crate::rtweekend::{degrees_to_radians, random_f64};
use crate::vec3::{cross, random_in_unit_disk, Point3};
use crate::{color::Color, hittable::Hittable, ray::Ray};
use std::f64::INFINITY;

use crate::color::{make_color, write_color};
use crate::hit_record::HitRecord;
use crate::vec3::Vec3;
use crate::vec3::{make_point, unit_vector, zero_vector};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub vfov: i32,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
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
            vfov: 90,
            lookfrom: make_point(0.0, 0.0, -1.0),
            lookat: make_point(0.0, 0.0, 0.0),
            vup: Vec3(0.0, 1.0, 0.0),
            u: zero_vector(),
            v: zero_vector(),
            w: zero_vector(),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            defocus_disk_u: zero_vector(),
            defocus_disk_v: zero_vector(),
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

        self.center = self.lookfrom;

        // Viewport Dimensions
        let theta = degrees_to_radians(self.vfov as f64);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width =
            viewport_height * (self.image_width as f64) / (self.image_height as f64);

        self.w = unit_vector(self.lookfrom - self.lookat);
        self.u = unit_vector(cross(self.vup, self.w));
        self.v = cross(self.w, self.u);

        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Find the upper left pixel
        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - (viewport_u / 2.0) - (viewport_v / 2.0);
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // Defocus disk basis vectors
        let defocus_radius = (self.focus_dist * degrees_to_radians(self.defocus_angle / 2.0)).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
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
        // Random sample camera ray for i,j from the defocus disk

        let pixel_center =
            self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };

        let ray_direction = pixel_sample - ray_origin;
        Ray {
            origin: ray_origin,
            direction: ray_direction,
        }
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }

    fn ray_color(&self, r: Ray, depth: i32, world: &dyn Hittable) -> Color {
        let mut rec = HitRecord::default();
        if depth <= 0 {
            return make_color(0.0, 0.0, 0.0);
        }
        if world.hit(&r, interval::new(0.001, INFINITY), &mut rec) {
            let mut scattered = Ray {
                origin: zero_vector(),
                direction: zero_vector(),
            };
            let mut attenuation = make_color(0.0, 0.0, 0.0);
            if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * self.ray_color(scattered, depth - 1, world);
            }
            return make_color(0.0, 0.0, 0.0);
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
