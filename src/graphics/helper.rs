use super::ray::Ray;


// Create a struct with the specified fields which implements vector arithmetic.
#[macro_export]
macro_rules! impl_vec_struct {
    (pub $type:ident => {
        $($fields:ident: $types:ty),*
    }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct $type {
            $(pub $fields: $types),*
        }

        impl_vec_struct!(@rest $type => {
            $($fields: $types),*
        });
    };

    ($type:ident => {
        $($fields:ident: $types:ty),*
    }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        struct $type {
            $(pub $fields: $types),*
        }
        
        impl_vec_struct!(@rest $type => {
            $($fields: $types),*
        });
    };

    (@rest $type:ident => {
        $($fields:ident: $types:ty),*
    }) => {
        // Implement vector addition.
        impl std::ops::Add for $type {
            type Output = $type;
            
            fn add(self, rhs: Self) -> Self::Output {
                $type {
                    $($fields: self.$fields + rhs.$fields,)*
                }
            }
        }

        impl std::ops::Add for &$type {
            type Output = $type;
            
            fn add(self, rhs: Self) -> Self::Output {
                $type {
                    $($fields: self.$fields + rhs.$fields,)*
                }
            }
        }

        impl std::ops::AddAssign for $type {
            fn add_assign(&mut self, rhs: Self) {
                *self = $type {
                    $($fields: self.$fields + rhs.$fields,)*
                }
            }
        }

        // Implement vector subtraction.
        impl std::ops::Sub for $type {
            type Output = $type;

            fn sub(self, rhs: Self) -> Self::Output {
                $type {
                    $($fields: self.$fields - rhs.$fields,)*
                }
            }
        }

        impl std::ops::Sub for &$type {
            type Output = $type;

            fn sub(self, rhs: Self) -> Self::Output {
                $type {
                    $($fields: self.$fields - rhs.$fields,)*
                }
            }
        }

        impl std::ops::SubAssign for $type {
            fn sub_assign(&mut self, rhs: Self) {
                *self = $type {
                    $($fields: self.$fields - rhs.$fields,)*
                }
            }
        }

        // Implement scalar multiplication.
        impl std::ops::Mul<f64> for $type {
            type Output = $type;
    
            fn mul(self, rhs: f64) -> Self::Output {
                $type {
                    $($fields: self.$fields * rhs,)*
                }
            }
        }
        
        impl std::ops::Mul<f64> for &$type {
            type Output = $type;
    
            fn mul(self, rhs: f64) -> Self::Output {
                $type {
                    $($fields: self.$fields * rhs,)*
                }
            }
        }
    
        impl std::ops::Mul<$type> for f64 {
            type Output = $type;
    
            fn mul(self, rhs: $type) -> Self::Output {
                rhs * self
            }
        }

        impl std::ops::Mul<&$type> for f64 {
            type Output = $type;
    
            fn mul(self, rhs: &$type) -> Self::Output {
                rhs * self
            }
        }
    
        impl std::ops::MulAssign<f64> for $type {
            fn mul_assign(&mut self, rhs: f64) {
                *self = $type {
                    $($fields: self.$fields * rhs,)*
                }
            }
        }

        // Implement scalar division.
        impl std::ops::Div<f64> for $type {
            type Output = $type;
    
            fn div(self, rhs: f64) -> Self::Output {
                $type {
                    $($fields: self.$fields / rhs,)*
                }
            }
        }

        impl std::ops::Div<f64> for &$type {
            type Output = $type;
    
            fn div(self, rhs: f64) -> Self::Output {
                $type {
                    $($fields: self.$fields / rhs,)*
                }
            }
        }
    
        impl std::ops::DivAssign<f64> for $type {
            fn div_assign(&mut self, rhs: f64) {
                *self = $type {
                    $($fields: self.$fields / rhs,)*
                }
            }
        }
    };
}
