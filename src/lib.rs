// TODO: check for arithmetic overflows and underflows in these functions.

pub mod graphics {
    const RGB_SCALE: f64 =  255.999;

    use std::{f64, ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign, Div, DivAssign}};
    use bmp::{Pixel, px, Image};
    // #[derive(Clone, Copy, Debug)]
    // pub struct Vec3(f64, f64, f64);

    // type Point = Vec3;
    
    macro_rules! impl_vec {
        (pub $type_name:ident => {
            $($fields:ident: $types:ty),*
        }) => {
            #[derive(Debug, Clone)]
            pub struct $type_name {
                $(pub $fields: $types),*
            }
        };

        (pub $type_name:ident => (
            $($types:ty),*
        )) => {
            #[derive(Debug, Clone)]
            pub struct $type_name (
                $(pub $types),*
            )
        };
        ($type_name:ident => {
            $($fields:ident: $types:ty),*
        }) => {
            #[derive(Debug, Clone)]
            struct $type_name {
                $(pub $fields: $types),*
            }
        };

        ($type_name:ident => (
            $($types:ty),*
        )) => {
            #[derive(Debug, Clone)]
            struct $type_name (
                $(pub $types),*
            )
        };
    }

    macro_rules! impl_vec_add {            
        ($type_name:ident => (
            $($fields:ident),*
        )) => {
            // Implement vector addition.
            impl Add for $type_name {
                type Output = Self;
                
                fn add(self, rhs: Self) -> Self::Output {
                    $type_name(
                        $(self.$fields + rhs.$fields,)*
                    )
                }
            }

            impl AddAssign for $type_name {
                fn add_assign(&mut self, rhs: Self) {
                    *self = $type_name(
                        $(self.$fields + rhs.$fields,)*
                    )
                }
            }
        };
    }

    macro_rules! impl_vec_sub {
        ($type_name:ident => (
            $($fields:ident),*
        )) => {
            // Implement vector subtraction.
            impl Sub for $type_name {
                type Output = Self;

                fn sub(self, rhs: Self) -> Self::Output {
                    $type_name(
                        $(self.$fields - rhs.$fields,)*
                    )
                }
            }

            impl SubAssign for $type_name {
                fn sub_assign(&mut self, rhs: Self) {
                    *self = $type_name(
                        $(self.$fields - rhs.$fields,)*
                    );
                }
            }
        };
    }

    macro_rules! impl_vec_mul {
        ($type_name:ident => (
            $($fields:ident),*
        )) => {        
            // Implement scalar multiplication.
            impl Mul<f64> for $type_name {
                type Output = Self;
        
                fn mul(self, rhs: f64) -> Self::Output {
                    $type_name(
                        $(self.$fields * rhs,)*
                    )
                }
            }
        
            impl Mul<$type_name> for f64 {
                type Output = $type_name;
        
                fn mul(self, rhs: $type_name) -> Self::Output {
                    rhs * self
                }
            }
        
            impl MulAssign<f64> for $type_name {
                fn mul_assign(&mut self, rhs: f64) {
                    *self = $type_name(
                        $(self.$fields * rhs,)*
                    );
                }
            }
        };
    }

    macro_rules! impl_vec_div {
        ($type_name:ident => (
            $($fields:ident),*
        )) => {
            // Implement scalar division.
            impl Div<f64> for $type_name {
                type Output = $type_name;
        
                fn div(self, rhs: f64) -> Self::Output {
                    $type_name(
                        $(self.$fields / rhs,)*
                    )
                }
            }
        
            impl DivAssign<f64> for $type_name {
                fn div_assign(&mut self, rhs: f64) {
                    *self = $type_name(
                        $(self.$fields / rhs,)*
                    );
                }
            }
        };
    }
    
    macro_rules! impl_vec_dot {
        (@sum $h:expr) => ($h);
        (@sum $h:expr, $($t:expr),*) => {
            ($h + impl_vec_dot!(@sum $($t),*));
        };
        ($type_name:ident => (
            $($fields:ident),*
        )) => {
            // Implement dot product.
            impl Mul for $type_name {
                type Output = f64;
        
                fn mul(self, rhs: Self) -> Self::Output {
                    impl_vec_dot!(@sum $(self.$fields * self.$fields),*);
                }
            }
        };
    }
    
    macro_rules! impl_vec_math {
        (@rest $type_name:ident => (
            $($fields:ident),*
        )) => {
            impl_vec_add!($type_name => ($($fields),*));
            impl_vec_sub!($type_name => ($($fields),*));
            impl_vec_mul!($type_name => ($($fields),*));
            impl_vec_div!($type_name => ($($fields),*));
            impl_vec_dot!($type_name => ($($fields),*));
        };

        // ! Assumes that there are three fields in the tuple struct
        // ! Currently trying to find a way to generalise this.
        (@rest_tuple $type_name:ident => (
            $($fields:ident),*
        )) => {
            impl_vec_add!($type_name => (0, 1, 2));
            impl_vec_sub!($type_name => (0, 1, 2));
            impl_vec_mul!($type_name => (0, 1, 2));
            impl_vec_div!($type_name => (0, 1, 2));
            impl_vec_dot!($type_name => (0, 1, 2));
        };

        (pub $type_name:ident => {
            $($fields:ident: $types:ty),*
        }) => {
            impl_vec!(pub $type_name:ident => {
                $($fields:ident: $types:ty),*
            });
            impl_vec_math!(@rest $type_name => ($($fields),*));
        };

        (pub $type_name:ident => (
            $($fields:ident),*
        )) => {
            impl_vec!(pub $type_name:ident => (
                $($fields:ident),*
            ));
            impl_vec_math!(@rest_tuple $type_name => ($($fields),*));
        };

        ($type_name:ident => {
            $($fields:ident: $types:ty),*
        }) => {
            impl_vec!(pub $type_name:ident => {
                $($fields:ident: $types:ty),*
            });
            impl_vec_math!(@rest $type_name => ($($fields),*));
        };

        ($type_name:ident => (
            $($types:ty),*
        )) => {
            impl_vec!(pub $type_name:ident => (
                $($types:ty),*
            ));
            impl_vec_math!(@rest_tuple $type_name => ($($fields),*));
        };
    }

    impl_vec_math!(pub Vec3 => (f64, f64, f64));

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

        pub fn new<T>(x: T, y: T, z: T) -> Self 
        where
            T: Into<f64>
        {
            Self (
                x.into(),
                y.into(),
                z.into(),
            )
        }

        pub fn zero() -> Self {
            Self(0.0, 0.0, 0.0)
        }

        pub fn print(&self) {
            println!("<{}, {}, {}>", self.x(), self.y(), self.z());
        }

        pub fn sqrlen(&self) -> f64 {
            (*self) * (*self)
        }

        // Handle floating point errors by using .max()
        pub fn len(&self) -> f64 {
            self.sqrlen().max(0.0).sqrt()
        }

        // Avoid division by zero by checking the edge case
        // TODO: Decide how to handle the zero vector. Should it return None or Vec3::zero()?
        pub fn unit(&self) -> Vec3 {
            if self.len() == 0.0 {
                Vec3::zero()
            } else {
                (*self) / self.len()
            }
        }
    }

    pub fn cross(vec1: &Vec3, vec2: &Vec3) -> Vec3 {
        Vec3(
            vec1.y() * vec2.z() - vec1.z() * vec2.y(),
            vec1.z() * vec2.x() - vec1.x() * vec2.z(),
            vec1.x() * vec2.y() - vec1.y() * vec2.x(),
        )
    }

    #[derive(Debug)]
    pub struct Ray {
        pub origin: Vec3,
        pub direction: Vec3,
    }

    impl Ray {
        pub fn at(&self, lambda: f64) -> Vec3 {
            self.origin + self.direction * lambda
        }

        // TODO: Add functionality (currently a stub)
        pub fn get_colour(&self) -> Pixel {
            px!(0, 0, 0)
        }

        pub fn lerp(&self) -> Pixel {
            let unit_vec = self.direction.unit();
            let y = 0.5 * (unit_vec.y() + 1.0);
            ((1.0 - y) * Colour::new(1.0, 1.0, 1.0) + y * Colour::new(0.5, 0.7, 1.0)).to_pixel()
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

    type Colour = Vec3;
    
    impl Colour {
        fn r(&self) -> f64 {
            self.0
        }
        
        fn g(&self) -> f64 {
            self.1
        }
        
        fn b(&self) -> f64 {
            self.2
        }

        fn to_pixel(&self) -> Pixel {
            // ! DEBUG STATEMENT - ensure no overflows
            assert!(self.is_valid());

            px!((self.r() * RGB_SCALE) as u8, (self.g() * RGB_SCALE) as u8, (self.b() * RGB_SCALE) as u8)
        }

        fn is_valid(&self) -> bool {
            (self.r() >= 0.0 && self.r() <= 255.0) && (self.g() >= 0.0 && self.g() <= 255.0) && (self.b >= 0.0 && self.b <= 255.0)
        }
    }

    impl Mul<Colour> for f64 {
        type Output = Colour;

        fn mul(self, rhs: Colour) -> Self::Output {
            Colour {
                r: self * (rhs.r as f64),
                g: self * (rhs.g as f64),
                b: self * (rhs.b as f64),
            }
        }
    }

    impl Mul<f64> for Colour {
        type Output = Colour;

        fn mul(self, rhs: f64) -> Self::Output {
            Colour {
                r: self.r as f64 * rhs,
                g: self.g as f64 * rhs,
                b: self.b as f64 * rhs,
            }
        }
    }

    impl Add for Colour {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Colour {
                r: self.r + rhs.r,
                g: self.g + rhs.g,
                b: self.b + rhs.b,
            }
        }
    }
}