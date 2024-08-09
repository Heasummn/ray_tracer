use crate::{ray::Hittable, util::Interval};

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &crate::ray::Ray, ray_t: Interval) -> Option<crate::ray::HitRecord> {
        let mut closest_so_far = ray_t.max;
        let mut hit_record: Option<crate::ray::HitRecord> = None;

        for object in self.objects.iter() {
            if let Some(hr) = object.hit(ray, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = hr.t;
                hit_record = Some(hr);
            }
        }

        hit_record
    }
}