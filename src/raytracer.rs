use crate::{camera::Camera, hittable_list::HittableList, ray::{self, Hittable}, util::Interval, vec3::Vec3};

pub struct Raytracer {
    camera: Camera,
    samples_per_pixel: u32,
    pixel_samples_scale: f64,
}

impl Raytracer {
    pub fn new(camera: Camera, samples_per_pixel: u32) -> Raytracer {
        Raytracer {
            camera,
            samples_per_pixel,
            pixel_samples_scale: 1.0 / samples_per_pixel as f64,
        }
    }

    fn ray_color(ray: ray::Ray, world: &HittableList) -> Vec3 {
        let t = world.hit(&ray, Interval::new(0.0, f64::INFINITY));
        if let Some(hr) = t {
            return ((0.5 * (hr.normal + Vec3::ones())));
        }

        let unit_dir = ray.direction.as_unit();
        let a = 0.5 * (unit_dir.y + 1.0);
        let rgb = (1.0 - a) * Vec3::ones() + a * Vec3 {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        };

        return rgb;
    }

    fn sample_square() -> Vec3 {
        Vec3 {
            x: rand::random::<f64>() - 0.5,
            y: rand::random::<f64>() - 0.5,
            z: 0.0,
        }

    }

    pub fn render(&self, world: &HittableList) {
        let mut dynamic_image = image::DynamicImage::new_rgb8(self.camera.image_width, self.camera.image_height);
        let imgbuf = dynamic_image.as_mut_rgb8().unwrap();

        for x in 0..self.camera.image_width {
            for y in 0..self.camera.image_height {
                let mut color: Vec3 = Vec3::zeros();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.camera.get_ray_with_offset(x, y, Self::sample_square());
                    color = color + Self::ray_color(ray, world);
                }
                imgbuf.put_pixel(x, y,  (self.pixel_samples_scale * color).into());
            }
        }
        imgbuf.save("output.png").unwrap();
    }
}