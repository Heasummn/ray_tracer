use crate::{camera::Camera, hittable_list::HittableList, materials::Scatterable, ray::{self, Hittable}, util::Interval, vec3::Vec3};

pub struct RaytracerBuilder {
    camera: Camera,
    samples_per_pixel: u32,
    max_depth: u32,
}

impl RaytracerBuilder {
    pub fn new(camera: Camera) -> RaytracerBuilder {
        RaytracerBuilder {
            camera,
            samples_per_pixel: 1,
            max_depth: 50,
        }
    }

    pub fn samples_per_pixel(mut self, samples_per_pixel: u32) -> RaytracerBuilder {
        self.samples_per_pixel = samples_per_pixel;
        self
    }

    pub fn max_depth(mut self, max_depth: u32) -> RaytracerBuilder {
        self.max_depth = max_depth;
        self
    }

    pub fn build(self) -> Raytracer {
        Raytracer::new(self.camera, self.samples_per_pixel, self.max_depth)
    }
}

pub struct Raytracer {
    camera: Camera,
    samples_per_pixel: u32,
    pixel_samples_scale: f64,
    max_depth: u32,
}

impl Raytracer {
    pub fn new(camera: Camera, samples_per_pixel: u32, max_depth: u32) -> Raytracer {
        Raytracer {
            camera,
            samples_per_pixel,
            pixel_samples_scale: 1.0 / samples_per_pixel as f64,
            max_depth,
        }
    }

    fn ray_color(ray: ray::Ray, world: &HittableList, depth: u32) -> Vec3 {
        if depth <= 0 {
            return Vec3::zeros();
        }

        let t = world.hit(&ray, Interval::new(0.001, f64::INFINITY));
        if let Some(hr) = t {
            if let Some((scattered, attenuation)) = hr.material.scatter(&ray, &hr) {
                return attenuation * Self::ray_color(scattered, world, depth - 1);
            }
            return Vec3::zeros();
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
                    let ray = self.camera.get_ray(x, y, Self::sample_square());
                    color = color + Self::ray_color(ray, world, self.max_depth);
                }
                imgbuf.put_pixel(x, y,  (self.pixel_samples_scale * color).into());
            }
        }
        imgbuf.save("output.png").unwrap();
    }
}