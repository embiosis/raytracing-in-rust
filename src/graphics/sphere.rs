use super::{material::Material, vec3::Vec3};

// TODO: Figure out the degree of precision for floating point comparisons.
const EPSILON: f64 = 0.000001;

#[derive(Clone, Default)]
pub struct Sphere {
    pub centre: Vec3,
    pub radius: f64,
    pub material: Material
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
    
    // Return the unit normal for a point if it lies on the sphere.
    pub fn normal(&self, point: Vec3) -> Option<Vec3> {
        let normal = point - self.centre;

        // Check if the point lies on the sphere.
        if normal.sqrlen() - self.radius * self.radius > EPSILON {
            None
        } else {
            Some(normal.normalise())
        }
    }

}