use crate::ray::{HitRecord, Ray};
use crate::vec3::Vec3;

use crate::ray::Hittable;

pub struct Sphere  {
  pub position: Vec3,
  pub radius: f64,
}

impl Sphere {
  pub fn new(position: Vec3, radius: f64) -> Sphere {
    Sphere {
      position,
      radius,
    }
  }
}

impl Hittable for Sphere {
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
    if (root <= t_min) || (root >= t_max) {
      root = (-h + sqrtd) / a;
      if (root <= t_min) || (root >= t_max) {
        return None
      }
    }

    return Some(HitRecord::new(root, 
      ray, 
      ((ray.at(root) - self.position) / self.radius)
    ));
  }

}
