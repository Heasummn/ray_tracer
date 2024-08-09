use crate::materials::Material;
use crate::ray::{HitRecord, Ray};
use crate::util::Interval;
use crate::vec3::Vec3;

use crate::ray::Hittable;

pub struct Sphere  {
  pub position: Vec3,
  pub radius: f64,
  pub material: Material,
}

impl Sphere {
  pub fn new(position: Vec3, radius: f64, material: Material) -> Sphere {
    Sphere {
      position,
      radius,
      material
    }
  }
}

impl Hittable for Sphere {
  fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
    let oc = ray.origin - self.position;
    let a = ray.direction.norm();
    let h = oc.dot(&ray.direction);
    let c = oc.norm() - (self.radius * self.radius);
    let disc = h*h - a*c;

    if disc < 0.0 {
      return None
    }

    let sqrtd = disc.sqrt();
    // Find the nearest root that lies in the acceptable range
    let mut root = (-h - sqrtd) / a;
    if !ray_t.surrounds(root) {
      root = (-h + sqrtd) / a;
      if !ray_t.surrounds(root) {
        return None
      }
    }

    return Some(HitRecord::new(root, 
      ray, 
      (ray.at(root) - self.position) / self.radius,
      &self.material
    ));
  }

}
