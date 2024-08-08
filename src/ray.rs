use crate::vec3::Vec3;

pub struct Ray {
  pub origin: Vec3,
  pub direction: Vec3,
}

impl Ray {
  pub fn new(origin: Vec3, direction: Vec3) -> Ray {
    Ray {
      origin,
      direction,
    }
  }

  pub fn point_at_parameter(&self, t: f64) -> Vec3 {
    self.origin + self.direction * t
  }
}

pub trait Hittable {
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool;
}