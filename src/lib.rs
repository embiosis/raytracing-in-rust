// TODO: check for arithmetic overflows and underflows in these functions.
pub mod vec3 {
    use std::{f64, ops::{Add, AddAssign, Mul, MulAssign}};
    #[derive(Clone, Copy)]
    pub struct Vec3(pub f64, pub f64, pub f64);
    
    // Implement vector addition.
    impl Add for Vec3 {
        type Output = Vec3;

        fn add(self, vec: Self) -> Self::Output {
            Vec3(
                self.x() + vec.x(),
                self.y() + vec.y(),
                self.z() + vec.z(),
            )
        }
    }

    impl Add for &Vec3 {
        type Output = Vec3;

        fn add(self, vec: Self) -> Self::Output {
            Vec3(
                self.x() + vec.x(),
                self.y() + vec.y(),
                self.z() + vec.z(),
            )
        }
    }

    impl AddAssign for Vec3 {
        fn add_assign(&mut self, vec: Self) {
            *self = Vec3(
                self.x() + vec.x(),
                self.y() + vec.y(),
                self.z() + vec.z(),
            );
        }
    }

    // Implement scalar multiplication.
    impl Mul<f64> for Vec3 {
        type Output = Vec3;

        fn mul(self, scalar: f64) -> Self::Output {
            Vec3(
                self.x() * scalar,
                self.y() * scalar,
                self.z() * scalar,
            )
        }
    }

    impl Mul<f64> for &Vec3 {
        type Output = Vec3;

        fn mul(self, scalar: f64) -> Self::Output {
            Vec3(
                self.x() * scalar,
                self.y() * scalar,
                self.z() * scalar,
            )
        }
    }

    impl MulAssign<f64> for Vec3 {
        fn mul_assign(&mut self, scalar: f64) {
            *self = Vec3(
                self.x() * scalar,
                self.y() * scalar,
                self.z() * scalar,
            );
        }
    }

    // Implement dot product.
    impl Mul for Vec3 {
        type Output = f64;

        fn mul(self, vec: Vec3) -> Self::Output {
            self.x() * vec.x() + self.y() + vec.y() + self.z() * vec.z()
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
            println!("<{} {} {}>", self.x(), self.y(), self.z());
        }

        pub fn len(&self) -> f64 {
            (self.x() * self.x() + self.y() * self.y() + self.z() * self.z()).sqrt()
        }

        pub fn unit(&self) -> Vec3 {
            *self * (1 as f64 / self.len())
        }
    }

    // TODO: check for arithmetic overflows / underflows?
    pub fn cross(vec1: Vec3, vec2: Vec3) -> Vec3 {
        Vec3(
            vec1.1 * vec2.2 - vec1.2 * vec2.1,
            vec1.2 * vec2.0 - vec1.0 * vec2.2,
            vec1.0 * vec2.1 - vec1.1 * vec2.0
        )
    }
}