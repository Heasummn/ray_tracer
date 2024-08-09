use crate::{ray::{HitRecord, Ray}, vec3::Vec3};

pub trait Scatterable {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)>;
}

pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl Scatterable for Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        match self {
            Material::Lambertian(l) => l.scatter(ray, hit),
            Material::Metal(m) => m.scatter(ray, hit),
        }
    }
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let mut scatter_direction = hit.normal + Vec3::random_in_unit_sphere().as_unit();

        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }

        let scattered = Ray::new(hit.p, scatter_direction);
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}

pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Metal {
        Metal {
            albedo,
        }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = ray.direction.as_unit().reflect(&hit.normal);
        let scattered = Ray::new(hit.p, reflected);
        let attenuation = self.albedo;
        Some((scattered, attenuation))
       
    }
}