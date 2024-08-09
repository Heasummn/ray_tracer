extern crate image;

mod camera;
mod ray;
mod raytracer;
mod vec3;
mod sphere;
mod hittable_list;

use core::f64;

use hittable_list::HittableList;
use sphere::Sphere;

use crate::ray::Hittable;
use crate::vec3::Vec3;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let width: u32 = 400;
    let camera = camera::Camera::new(Vec3::zeros(), aspect_ratio, width, 2.0, 1.0);

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));


    let raytracer = raytracer::Raytracer::new(camera);
    raytracer.render(&world);
}
