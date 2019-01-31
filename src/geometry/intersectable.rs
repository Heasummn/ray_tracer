use crate::ray::Ray;

pub trait Intersectable {
    fn intersects(&self, ray: &Ray) -> bool;
}