use crate::{ray, vec3};

pub struct HitRecord {
    pub p: vec3::Point3,
    pub normal: vec3::Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            p: vec3::zero_vector(),
            normal: vec3::zero_vector(),
            t: 0.0,
            front_face: false,
        }
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: ray::Ray, outward_normal: vec3::Vec3) {
        self.front_face = vec3::dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
