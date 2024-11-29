use crate::impl_vec_struct;
use std::fmt::Display;

// Create a Vec3 struct with x, y, z fields and basic arithmetic operations.
impl_vec_struct!(pub Vec3 => {x: f64, y: f64, z: f64});

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec3<{}, {}, {}>", self.x, self.y, self.z)
    }
}

// Implement dot product.
impl std::ops::Mul for Vec3 {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x +
        self.y * rhs.y +
        self.z * rhs.z
    }
}

impl Vec3 {
    pub fn new<T>(x: T, y: T, z: T) -> Self 
    where
        T: Into<f64>
    {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    pub fn zero() -> Self {
        Self::new(0, 0, 0)
    }

    pub fn sqrlen(&self) -> f64 {
        (*self) * (*self)
    }

    // Handle floating point errors by using .max()
    // Technically, sqrlen cannot return NaN because no square operations generate NaN.
    // See https://en.wikipedia.org/wiki/NaN#Operations_generating_NaN for more details.
    pub fn len(&self) -> f64 {
        self.sqrlen().max(0.0).sqrt()
    }

    // Avoid division by zero by checking the edge case
    // TODO: Decide how to handle the zero vector. Should it return None or Vec3::zero()?
    pub fn normalise(&self) -> Vec3 {
        if self.len() == 0.0 {
            panic!("Division by zero!")
        } else {
            (*self) / self.len()
        }
    }

    pub fn cross(vec1: &Vec3, vec2: &Vec3) -> Vec3 {
        Vec3 {
            x: vec1.y * vec2.z - vec1.z * vec2.y,
            y: vec1.z * vec2.x - vec1.x * vec2.z,
            z: vec1.x * vec2.y - vec1.y * vec2.x,
        }
    }
}