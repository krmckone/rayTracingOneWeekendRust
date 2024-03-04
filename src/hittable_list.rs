use crate::{hit_record::HitRecord, hittable::Hittable, ray::Ray};

struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    fn clear(&mut self) {
        self.objects.clear();
    }

    fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object)
    }

    fn hit(&self, r: Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for object in self.objects.iter() {
            if object.hit(&r, ray_tmin, closest_so_far, temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec = temp_rec;
            }
        }

        hit_anything
    }
}
