use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
  // Eventually allow for moving of camera
  pub width: u32,
  pub height: u32,
  pub fov: f64,
  aspect_ratio: f64,
}

impl Camera {
  pub fn new(width: u32, height: u32, fov: f64) -> Camera {
    return Camera {
      width: width,
      height: height,
      fov: fov,
      aspect_ratio: width as f64 / height as f64,
    };
  }

  // Makes use of basic trig to convert x/y to camera coords for ray
  pub fn get_ray(self, x: u32, y: u32) -> Ray {
    let fov_correction: f64 = (self.fov.to_radians() / 2.0).tan();
    return Ray {
      origin: Vec3::zeros(),
      direction: Vec3 {
        x: (2.0 * ((x as f64 + 0.5) / self.width as f64) - 1.0) // converting (0, width) -> (-1, 1)
          * fov_correction
          * self.aspect_ratio, // correcting square size for non-square images
        y: (1.0 - 2.0 * (y as f64 + 0.5) / self.height as f64),
        z: -1.0,
      }
      .as_unit(),
    };
  }
}
