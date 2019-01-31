use crate::Intersectable;
use crate::ray::Ray;

pub struct GeometryList {
    pub objs: Vec<Box<Intersectable>>
}

impl Intersectable for GeometryList {
    fn intersects(&self, ray: &Ray) -> bool {
        // do stuff here
        return true;
    }
}