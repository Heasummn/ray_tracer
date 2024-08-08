use crate::ray::Ray;
use crate::vec3::Vec3;

use crate::ray::Hittable;

pub struct Sphere  {
  pub position: Vec3,
  pub radius: f64,
}

impl Hittable for Sphere {
  fn hit(&self, ray: &Ray, _t_min: f64, _t_max: f64) -> bool {
    let oc = ray.origin - self.position;
    let a = ray.direction.dot(&ray.direction);
    let b = oc.dot(&ray.direction);
    let c = oc.dot(&oc) - (self.radius * self.radius);
    let disc = b * b - a*c;

    disc > 0.0
  }
}
