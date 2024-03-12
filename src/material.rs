use crate::{
    color::{make_color, Color},
    hit_record::HitRecord,
    ray::Ray,
    rtweekend::random_f64,
    vec3::{dot, random_unit, reflect, refract, unit_vector},
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

#[derive(Clone)]
pub struct Dielectric {
    ir: f64,
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = make_color(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = unit_vector(r_in.direction());
        let cos_theta = dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || self.reflectance(cos_theta, refraction_ratio) > random_f64() {
                reflect(unit_direction, rec.normal)
            } else {
                refract(unit_direction, rec.normal, refraction_ratio)
            };
        *scattered = Ray {
            origin: rec.p,
            direction,
        };
        true
    }
}

impl Dielectric {
    fn reflectance(&self, cosine: f64, ref_idx: f64) -> f64 {
        // Schlick's approximation
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

pub fn make_metal(albedo: Color, fuzz: f64) -> Metal {
    Metal { albedo, fuzz }
}

pub fn make_dielectric(ir: f64) -> Dielectric {
    Dielectric { ir }
}
