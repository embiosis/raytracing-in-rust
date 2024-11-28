// TODO: check for arithmetic overflows and underflows in these functions.

pub mod graphics {
    const RGB_SCALE: f64 =  255.999;

    use std::{f64, fmt::Display, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}};
    use bmp::{Pixel, px, Image};
    #[derive(Clone, Copy, Debug)]
    pub struct Vec3<T: Numeric>(pub T, pub T, pub T);

    macro_rules! numeric_impl {
        ($($types:ty),*) => {
            // Create a marker trait to mark numeric types
            pub trait Numeric: 
                Clone + 
                Copy + 
                Add<Output = Self> + 
                Mul<Output = Self> + 
                Div<Output = Self> + 
                Sub<Output = Self> +
                PartialOrd +
                Display
            {}
            $(impl Numeric for $types {})*
        };
    }

    numeric_impl!(f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

    // Implement vector addition.
    impl<T: Numeric> Add for Vec3<T> {
        type Output = Vec3<T>;

        fn add(self, rhs: Self) -> Self::Output {
            Vec3(
                self.x() + rhs.x(),
                self.y() + rhs.y(),
                self.z() + rhs.z(),
            )
        }
    }

    impl<T: Numeric> AddAssign for Vec3<T> {
        fn add_assign(&mut self, rhs: Self) {
            *self = Vec3(
                self.x() + rhs.x(),
                self.y() + rhs.y(),
                self.z() + rhs.z(),
            );
        }
    }

    // Implement vector subtraction.
    impl<T: Numeric> Sub for Vec3<T> {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Vec3(
                self.x() - rhs.x(),
                self.y() - rhs.y(),
                self.z() - rhs.z(),
            )
        }
    }

    impl<T: Numeric> SubAssign for Vec3<T> {
        fn sub_assign(&mut self, rhs: Self) {
            *self = Vec3(
                self.x() - rhs.x(),
                self.y() - rhs.y(),
                self.z() - rhs.z(),
            );
        }
    }

    // Implement dot product.
    impl<T: Numeric> Mul<Vec3<T>> for Vec3<T> {
        type Output = T;

        fn mul(self, rhs: Self) -> Self::Output {
            self.x() * rhs.x() + self.y() + rhs.y() + self.z() * rhs.z()
        }
    }

    // Implement scalar multiplication.
    // TODO: Is making multiplication this generic a bad idea?
    impl<T: Numeric> Mul<T> for Vec3<T> {
        type Output = Vec3<T>;

        fn mul(self, rhs: T) -> Self::Output {
            Vec3(
                self.x() * rhs,
                self.y() * rhs,
                self.z() * rhs,
            )
        }
    }

    impl<T: Numeric> Mul<Vec3<T>> for T {
        type Output = Vec3<T>;

        fn mul(self, rhs: Vec3<T>) -> Self::Output {
            rhs * self
        }
    }

    impl<T: Numeric> MulAssign<T> for Vec3<T> {
        fn mul_assign(&mut self, rhs: T) {
            *self = Vec3(
                self.x() * rhs,
                self.y() * rhs,
                self.z() * rhs,
            );
        }
    }

    // Implement scalar division.
    impl<T: Numeric> Div<T> for Vec3<T> {
        type Output = Vec3<T>;

        fn div(self, rhs: T) -> Self::Output {
            Vec3(
                self.x() / rhs,
                self.y() / rhs,
                self.z() / rhs,
            )
        }
    }

    impl<T: Numeric> DivAssign<T> for Vec3<T> {
        fn div_assign(&mut self, rhs: T) {
            *self = Vec3(
                self.x() / rhs,
                self.y() / rhs,
                self.z() / rhs,
            );
        }
    }

    impl<T: Numeric> Vec3<T> {
        pub fn x(&self) -> T {
            self.0
        }

        pub fn y(&self) -> T {
            self.1
        }

        pub fn z(&self) -> T {
            self.2
        }

        pub fn new(x: T, y: T, z: T) -> Self {
            Self (
                x,
                y,
                z,
            )
        }

        pub fn zero_i32() -> Vec3<i32> {
            Vec3(0, 0, 0)
        }

        pub fn zero_f64() -> Vec3<f64> {
            Vec3(0.0, 0.0, 0.0)
        }

        pub fn print(&self) {
            println!("<{}, {}, {}>", self.x(), self.y(), self.z());
        }

        pub fn sqrlen(&self) -> T {
            (*self) * (*self)
        }

        // Get the length of a vector.
        // If any floating point errors are encountered, returns 0.0f.
        pub fn len(&self) -> f64 {
            self.sqrlen().max(0.0).sqrt()
        }

        // Normalises a vector into its unit vector.
        // TODO: Decide on the functionality of this function if Vec3 is the zero vector.
        // TODO: Should it return None or Vec3::zero()?
        // pub fn unit(&self) -> Option<Vec3> {
        //     if self.len() == 0.0 {
        //         None
        //     } else {
        //         Some((*self) / self.len())
        //     }
        // }
        pub fn unit(&self) -> Vec3<T> {
            if self.len() == 0.0 {
                Vec3::zero()
            } else {
                (*self) / self.len()
            }
        }
    }

    // Implement cross product
    pub fn cross<T: Clone>(vec1: &Vec3<T>, vec2: &Vec3<T>) -> Vec3<T> {
        Vec3(
            vec1.y() * vec2.z() - vec1.z() * vec2.y(),
            vec1.z() * vec2.x() - vec1.x() * vec2.z(),
            vec1.x() * vec2.y() - vec1.y() * vec2.x(),
        )
    }

    #[derive(Debug)]
    pub struct Ray<T: Clone> {
        pub origin: Vec3<T>,
        pub direction: Vec3<T>,
    }

    impl<T: Clone> Ray<T> {
        pub fn at(&self, lambda: f64) -> Vec3<T> {
            self.origin + self.direction * lambda
        }

        // TODO: Add functionality (currently a stub)
        pub fn get_colour(&self) -> Pixel {
            px!(0, 0, 0)
        }

        // TODO: Implement actual values on this.
        pub fn lerp(&self,) -> Pixel {
            let unit_vec = self.direction.unit();
            let y = 0.5 * (unit_vec.y() + 1.0);
            ((1.0 - y) * MyPixel::new(1.0, 1.0, 1.0) + y * MyPixel::new(0.5, 0.7, 1.0)).to_pixel()
        }
    }
    
    pub struct Viewport {
        pub image_height: u32,
        pub image_width: u32,
        pub focal_length: f64,
        pub viewport_height: f64,
        pub viewport_width: f64,
        pub camera_center: Vec3,
        pub viewport_center: Vec3,
        pub viewport_i: Vec3,
        pub viewport_j: Vec3,
        pub pixel_delta_i: Vec3,
        pub pixel_delta_j: Vec3,
        pub viewport_upper_left: Vec3,
    }

    impl Viewport {
        // TODO: Error checking to ensure pixel coord is within bounds?
        pub fn get_pixel_coord(&self, x: u32, y: u32) -> Vec3 {
            self.viewport_upper_left + (0.5 + x as f64) * self.pixel_delta_i + (0.5 + y as f64) * self.pixel_delta_j
        }

        // TODO: Refactor this to make it less messy (reuse previous values)
        pub fn new(image_width: u32, focal_length: f64, viewport_height: f64, aspect_ratio: f64, camera_center: Vec3) -> Viewport {
            Viewport {
                image_height: std::cmp::max((image_width as f64 / aspect_ratio) as u32, 1),
                image_width,
                focal_length,
                viewport_height,
                viewport_width: viewport_height * aspect_ratio,
                camera_center,
                viewport_center: Vec3(0.0, 0.0, focal_length),
                viewport_i: Vec3(viewport_height, 0.0, 0.0),
                viewport_j: Vec3(0.0, -viewport_height, 0.0), // invert y-axis due to orientation of viewport coordinates
                pixel_delta_i: Vec3(viewport_height, 0.0, 0.0) / image_width as f64 / aspect_ratio,
                pixel_delta_j: Vec3(0.0, -viewport_height, 0.0) / (image_width as f64),
                viewport_upper_left: camera_center - Vec3(0.0, 0.0, focal_length) - Vec3(viewport_height, 0.0, 0.0) / 2.0 - Vec3(0.0, -viewport_height, 0.0) / 2.0
            }
        }

        pub fn render(&self, save_path: String) {
            let mut img = Image::new(self.image_width, self.image_height);

            for y in 0..self.image_height {
                for x in 0..self.image_width {
                    let pixel_center = self.get_pixel_coord(x, y);
                    let ray_dir = pixel_center - self.camera_center;
                    let ray = Ray { origin: self.camera_center, direction: ray_dir };
                    img.set_pixel(x, y, ray.lerp()); 
                }
            }

            let _ = img.save(save_path);
        }
    }

    #[derive(Debug)]
    pub struct MyPixel {
        r: f64,
        g: f64,
        b: f64,
    }

    impl MyPixel {
        fn new(r: f64, g: f64, b: f64) -> Self {
            MyPixel {
                r,
                g,
                b,
            }
        }

        fn is_valid_rgb(&self) -> bool {
            (self.r >= 0.0 && self.r <= 255.0) && (self.g >= 0.0 && self.g <= 255.0) && (self.b >= 0.0 && self.b <= 255.0)
        }
    }

    impl From<MyPixel> for Pixel {
        fn from(value: MyPixel) -> Self {
            // ! DEBUG STATEMENT - ensure value is a valid RGB pixel.
            assert!(value.is_valid_rgb());

            px!((value.r * RGB_SCALE) as u8, (value.g * RGB_SCALE) as u8, (value.b * RGB_SCALE) as u8)
        }
    }

    impl Mul<MyPixel> for f64 {
        type Output = MyPixel;

        fn mul(self, rhs: MyPixel) -> Self::Output {
            MyPixel {
                r: self * (rhs.r as f64),
                g: self * (rhs.g as f64),
                b: self * (rhs.b as f64),
            }
        }
    }

    impl Mul<f64> for MyPixel {
        type Output = MyPixel;

        fn mul(self, rhs: f64) -> Self::Output {
            MyPixel {
                r: self.r as f64 * rhs,
                g: self.g as f64 * rhs,
                b: self.b as f64 * rhs,
            }
        }
    }

    impl Add for MyPixel {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            MyPixel {
                r: self.r + rhs.r,
                g: self.g + rhs.g,
                b: self.b + rhs.b,
            }
        }
    }
}