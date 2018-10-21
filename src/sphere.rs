use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
  pub position: Vec3,
  pub radius: f64,
}

impl Sphere {
  pub fn intersects(&self, ray: &Ray) -> bool {
    let length = self.position - ray.origin;
    let adj = length.dot(&ray.direction);
    let distance = length.dot(&length) - (adj * adj);

    return distance < (self.radius * self.radius);
  }
}
