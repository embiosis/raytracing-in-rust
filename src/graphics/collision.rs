use super::{ray::Ray, vec3::Vec3};

#[derive(Clone)]
pub struct CollisionInfo {
    point: Vec3,
    normal: Vec3,
    lambda: f64,
}

pub trait Collision {
    fn hit(&self, ray: Ray) -> bool;
}