use std::fmt;
use std::ops;

use crate::rtweekend::random_f64;
use crate::rtweekend::random_f64_in_range;

#[derive(Copy, Clone)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl fmt::Display for Vec3 {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = self.x();
        let y = self.y();
        let z = self.z();
        print!("<Vec3: {x} {y} {z}>");
        Ok(())
    }
}

impl ops::Index<i32> for Vec3 {
    type Output = f64;

    fn index(&self, index: i32) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Vec3 index out of bounds {index}"),
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(-self[0], -self[1], -self[2])
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2])
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = Self(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2])
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2])
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = Self(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2])
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2])
    }
}

impl ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        *self = Self(self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2])
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self * rhs[0], self * rhs[1], self * rhs[2])
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u[0] * v[0] + u[1] * v[1] + u[2] * v[2]
}

fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3(
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0],
    )
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

impl Vec3 {
    pub fn x(&self) -> f64 {
        self[0]
    }

    pub fn y(&self) -> f64 {
        self[1]
    }

    pub fn z(&self) -> f64 {
        self[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (f64::abs(self[0]) < s) && (f64::abs(self[1]) < s) && (f64::abs(self[2]) < s)
    }
}

pub fn random() -> Vec3 {
    Vec3(random_f64(), random_f64(), random_f64())
}

pub fn random_in_range(min: f64, max: f64) -> Vec3 {
    Vec3(
        random_f64_in_range(min, max),
        random_f64_in_range(min, max),
        random_f64_in_range(min, max),
    )
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_in_range(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}

pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let on_unit_sphere = random_unit();
    if dot(on_unit_sphere, normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * n
}

pub type Point3 = Vec3;

pub fn make_point(x: f64, y: f64, z: f64) -> Point3 {
    Vec3(x, y, z)
}

pub fn zero_vector() -> Vec3 {
    Vec3(0.0, 0.0, 0.0)
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn it_creates_a_vec3() {
        let v = Vec3(0.0, 0.1, 0.2);
        assert_eq!(v.0, 0.0);
        assert_eq!(v.1, 0.1);
        assert_eq!(v.2, 0.2);
    }
}
