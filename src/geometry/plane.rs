use crate::Intersectable;
use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Plane {
    pub center: Vec3,
    pub norm: Vec3
}

impl Intersectable for Plane {
    fn intersects(&self, ray: &Ray) -> bool {
        let normal = &self.norm;
        let denom = normal.dot(&ray.direction);
        if denom > 1e-6 {
            let v = self.center - ray.origin;
            let distance = v.dot(&normal) / denom;
            if distance >= 0.0 {
                return true;
            }
        }
        false

    }
}