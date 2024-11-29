use super::{material::Material, ray::Ray, vec3::Vec3};

// TODO: Figure out the degree of precision for floating point comparisons.
const EPSILON: f64 = 0.000001;

#[derive(Clone, Default)]
pub struct Sphere {
    centre: Vec3,
    radius: f64,
    material: Material
}

impl Sphere {
    pub fn new<T, U>(x: T, y: T, z: T, radius: U, material: Option<Material>) -> Sphere
    where 
        T: Into<f64>,
        U: Into<f64>
    {
        Sphere {
            centre: Vec3::new(x, y, z),
            radius: radius.into(),
            material: match material {
                Some(material) => material,
                None => Material::default(),
            },
        }
    }
    pub fn hit(&self, ray: &Ray) -> Option<f64> {
        let position_vec = self.centre - ray.origin;
        
        // Use the quadratic formula to check for intersections
        // between the ray and the sphere.
        let a = ray.direction * ray.direction;
        let b = -2.0 * (ray.direction * position_vec);
        let c = position_vec * position_vec - self.radius * self.radius;

        let discriminant = b * b - (4.0 * a * c);
        
        if discriminant < 0.0 {
            None
        } else {
            Some( (-b - discriminant.sqrt()) / (2.0 * a) )
        }
    }
}