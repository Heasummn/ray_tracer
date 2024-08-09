use crate::{materials::Material, util::Interval, vec3::Vec3};

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

  pub fn at(&self, t: f64) -> Vec3 {
    self.origin + self.direction * t
  }
}

pub struct HitRecord<'material> {
  pub p: Vec3,
  pub normal: Vec3,
  pub t: f64,
  pub front_face: bool,
  pub material: &'material Material,
}

impl<'a> HitRecord<'a> {
  pub fn new(t: f64, ray: &Ray, outward_normal: Vec3, material: &'a Material) -> HitRecord<'a> {
    let front_face = ray.direction.dot(&outward_normal) < 0.0;
    let normal = if front_face { outward_normal } else { -outward_normal };
    HitRecord {
      t,
      p: ray.at(t),
      normal,
      front_face,
      material,
    }
  }
}

pub trait Hittable {
  fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}