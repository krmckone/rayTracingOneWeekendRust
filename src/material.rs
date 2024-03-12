use crate::{
    color::Color,
    hit_record::HitRecord,
    ray::Ray,
    vec3::{random_unit, reflect, unit_vector},
};

pub trait Material: MaterialClone {
    fn scatter(
        &self,
        r_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

trait MaterialClone {
    fn clone_box(&self) -> Box<dyn Material>;
}

impl<T> MaterialClone for T
where
    T: 'static + Material + Clone,
{
    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct Lambertian(Color);

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray {
            origin: rec.p,
            direction: scatter_direction,
        };
        *attenuation = self.0;
        true
    }
}

pub fn make_lambertian(albedo: Color) -> Lambertian {
    Lambertian(albedo)
}

#[derive(Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(unit_vector(r_in.direction()), rec.normal);
        *scattered = Ray {
            origin: rec.p,
            direction: reflected + self.fuzz * random_unit(),
        };
        *attenuation = self.albedo;
        true
    }
}

pub fn make_metal(albedo: Color, fuzz: f64) -> Metal {
    Metal { albedo, fuzz }
}
