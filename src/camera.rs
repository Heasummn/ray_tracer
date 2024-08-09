use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
  pub origin: Vec3,
  // aspect_ratio: f64,
  pub image_width: u32,
  pub image_height: u32,
  // focal_length: f64,
  // viewport_height: f64,
  // viewport_width: f64,
  pixel_delta_u: Vec3,
  pixel_delta_v: Vec3,
  pixel00_loc: Vec3,
}

impl Camera {
  pub fn new(origin: Vec3, aspect_ratio: f64, image_width: u32, viewport_height: f64, focal_length: f64) -> Camera {
    let viewport_width = aspect_ratio * viewport_height;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    let viewport_u = Vec3 { x: viewport_width, y: 0.0, z: 0.0 };
    let viewport_v = Vec3 { x: 0.0, y: -viewport_height, z: 0.0 };

    let pixel_delta_u = viewport_u / (image_width as f64);
    let pixel_delta_v = viewport_v / (image_height as f64);
    let viewport_upper_left = origin - (Vec3 {x: 0.0, y: 0.0, z: focal_length}) - viewport_u/2.0 - viewport_v/2.0;
    let pixel00_loc = viewport_upper_left + pixel_delta_u/2.0 + pixel_delta_v/2.0;
    

    Camera {
      origin,
      image_width,
      image_height,
      pixel_delta_u,
      pixel_delta_v,
      pixel00_loc,
    }
  }

  pub fn get_ray(&self, x: u32, y: u32, offset: Vec3) -> Ray {
    let pixel_center = self.pixel00_loc + ((offset.x + x as f64) * self.pixel_delta_u) + ((offset.y + y as f64) * self.pixel_delta_v);
    let direction = pixel_center - self.origin;

    return Ray::new(self.origin, direction);
  }
}