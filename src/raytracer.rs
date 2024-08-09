use crate::{camera::Camera, hittable_list::HittableList, ray::{self, Hittable}, util::Interval, vec3::Vec3};

pub struct Raytracer {
    camera: Camera,
}

impl Raytracer {
  pub fn new(camera: Camera) -> Raytracer {
    Raytracer {
        camera
    }
  }

    fn ray_color(ray: ray::Ray, world: &HittableList) -> image::Rgb<u8> {
        let t = world.hit(&ray, Interval::new(0.0, f64::INFINITY));
        if let Some(hr) = t {
            return ((0.5 * (hr.normal + Vec3::ones()))).into();
        }

        let unit_dir = ray.direction.as_unit();
        let a = 0.5 * (unit_dir.y + 1.0);
        let rgb = (1.0 - a) * Vec3::ones() + a * Vec3 {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        };

        return rgb.into();
    }

  pub fn render(&self, world: &HittableList) {
    let mut dynamic_image = image::DynamicImage::new_rgb8(self.camera.image_width, self.camera.image_height);
    let imgbuf = dynamic_image.as_mut_rgb8().unwrap();

    for x in 0..self.camera.image_width {
        for y in 0..self.camera.image_height {
            let ray = self.camera.get_ray(x, y);
            imgbuf.put_pixel(x, y, Self::ray_color(ray, world).into());
        }
    }
    imgbuf.save("output.png").unwrap();
  }
}