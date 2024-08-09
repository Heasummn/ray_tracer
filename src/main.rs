extern crate image;

mod camera;
mod ray;
mod raytracer;
mod vec3;
mod sphere;
mod hittable_list;
mod util;
mod materials;

use hittable_list::HittableList;
use materials::{Lambertian, Material, Metal};
use sphere::Sphere;
use raytracer::RaytracerBuilder;

use crate::vec3::Vec3;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let width: u32 = 400;
    let camera = camera::Camera::new(Vec3::zeros(), aspect_ratio, width, 2.0, 1.0);

    
    let mut world = HittableList::new();

    let material_ground = Material::Lambertian(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_center = Material::Lambertian(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let material_left = Material::Metal(Metal::new(Vec3::new(0.8, 0.8, 0.8)));
    let material_right = Material::Metal(Metal::new(Vec3::new(0.8, 0.6, 0.2)));

    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5, material_center)));
    world.add(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right)));


    let raytracer = RaytracerBuilder::new(camera).samples_per_pixel(100).max_depth(50).build();
    raytracer.render(&world);
}
