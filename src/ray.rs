use crate::vec3::*;
pub struct Ray {
    orgin: Point3,
    direction: Vec3,
}

impl Ray {
    fn origin(self) -> Point3 {
        self.orgin
    }

    fn direction(self) -> Vec3 {
        self.orgin
    }

    fn at(self, t: f64) -> Point3 {
        self.orgin + t * &self.direction
    }
}
