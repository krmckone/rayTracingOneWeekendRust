use crate::vec3::*;
pub struct Ray {
    pub(crate) orgin: Point3,
    pub(crate) direction: Vec3,
}

impl Ray {
    pub fn origin(&self) -> Point3 {
        self.orgin
    }

    pub fn direction(&self) -> Vec3 {
        self.orgin
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orgin + t * self.direction
    }
}
