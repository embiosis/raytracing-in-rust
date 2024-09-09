// TODO: check for arithmetic overflows and underflows in these functions.
// TODO: Remove impls for &Vec3 (dereference manually when needed instead).

pub mod vec3 {
    use std::{f64, ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign, Div, DivAssign}};
    #[derive(Clone, Copy)]
    pub struct Vec3(pub f64, pub f64, pub f64);
    
    // Implement vector addition.
    impl Add for Vec3 {
        type Output = Vec3;

        fn add(self, rhs: Self) -> Self::Output {
            Vec3(
                self.x() + rhs.x(),
                self.y() + rhs.y(),
                self.z() + rhs.z(),
            )
        }
    }

    impl AddAssign for Vec3 {
        fn add_assign(&mut self, rhs: Self) {
            *self = Vec3(
                self.x() + rhs.x(),
                self.y() + rhs.y(),
                self.z() + rhs.z(),
            );
        }
    }

    // Implement vector subtraction.
    impl Sub for Vec3 {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Vec3(
                self.x() - rhs.x(),
                self.y() - rhs.y(),
                self.z() - rhs.z(),
            )
        }
    }

    impl SubAssign for Vec3 {
        fn sub_assign(&mut self, rhs: Self) {
            *self = Vec3(
                self.x() - rhs.x(),
                self.y() - rhs.y(),
                self.z() - rhs.z(),
            );
        }
    }

    // Implement dot product.
    impl Mul for Vec3 {
        type Output = f64;

        fn mul(self, rhs: Self) -> Self::Output {
            self.x() * rhs.x() + self.y() + rhs.y() + self.z() * rhs.z()
        }
    }

    // Implement scalar multiplication.
    impl Mul<f64> for Vec3 {
        type Output = Vec3;

        fn mul(self, rhs: f64) -> Self::Output {
            Vec3(
                self.x() * rhs,
                self.y() * rhs,
                self.z() * rhs,
            )
        }
    }

    impl Mul<Vec3> for f64 {
        type Output = Vec3;

        fn mul(self, rhs: Vec3) -> Self::Output {
            rhs * self
        }
    }

    impl MulAssign<f64> for Vec3 {
        fn mul_assign(&mut self, rhs: f64) {
            *self = Vec3(
                self.x() * rhs,
                self.y() * rhs,
                self.z() * rhs,
            );
        }
    }

    // Implement scalar division.
    impl Div<f64> for Vec3 {
        type Output = Vec3;

        fn div(self, rhs: f64) -> Self::Output {
            Vec3(
                self.x() / rhs,
                self.y() / rhs,
                self.z() / rhs,
            )
        }
    }

    impl DivAssign<f64> for Vec3 {
        fn div_assign(&mut self, rhs: f64) {
            *self = Vec3(
                self.x() / rhs,
                self.y() / rhs,
                self.z() / rhs,
            );
        }
    }

    impl Vec3 {
        pub fn x(&self) -> f64 {
            self.0
        }

        pub fn y(&self) -> f64 {
            self.1
        }

        pub fn z(&self) -> f64 {
            self.2
        }

        pub fn print(&self) {
            println!("<{}, {}, {}>", self.x(), self.y(), self.z());
        }

        pub fn sqrlen(&self) -> f64 {
            (*self) * (*self)
        }

        pub fn len(&self) -> f64 {
            self.sqrlen().sqrt()
        }

        pub fn unit(&self) -> Vec3 {
            (*self) / self.len()
        }
    }

    pub fn cross(vec1: &Vec3, vec2: &Vec3) -> Vec3 {
        Vec3(
            vec1.y() * vec2.z() - vec1.z() * vec2.y(),
            vec1.z() * vec2.x() - vec1.x() * vec2.z(),
            vec1.x() * vec2.y() - vec1.y() * vec2.x(),
        )
    }
}

pub mod ray {
    use crate::vec3::Vec3;

    pub struct Ray {
        pub origin: Vec3,
        pub direction: Vec3,
    }

    impl Ray {
        fn at(&self, lambda: f64) -> Vec3 {
            self.origin + self.direction * lambda
        }
    }


}