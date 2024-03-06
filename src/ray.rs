use crate::vec3::*;
pub struct Ray {
    pub(crate) origin: Point3,
    pub(crate) direction: Vec3,
}

impl Ray {
    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
