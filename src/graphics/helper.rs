
// Create a struct with the specified fields which implements vector arithmetic.
#[macro_export]
macro_rules! impl_vec_struct {
    (pub $type_name:ident => {
        $($fields:ident: $types:ty),*
    }) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct $type_name {
            $(pub $fields: $types),*
        }

        impl_vec_struct!(@rest $type_name => {
            $($fields: $types),*
        });
    };

    ($type_name:ident => {
        $($fields:ident: $types:ty),*
    }) => {
        #[derive(Debug, Clone, Copy)]
        struct $type_name {
            $(pub $fields: $types),*
        }
        
        impl_vec_struct!(@rest $type_name => {
            $($fields: $types),*
        });
    };

    (@rest $type_name:ident => {
        $($fields:ident: $types:ty),*
    }) => {
        // Implement vector addition.
        impl std::ops::Add for $type_name {
            type Output = Self;
            
            fn add(self, rhs: Self) -> Self::Output {
                $type_name {
                    $($fields: self.$fields + rhs.$fields,)*
                }
            }
        }

        impl std::ops::AddAssign for $type_name {
            fn add_assign(&mut self, rhs: Self) {
                *self = $type_name {
                    $($fields: self.$fields + rhs.$fields,)*
                }
            }
        }

        // Implement vector subtraction.
        impl std::ops::Sub for $type_name {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                $type_name {
                    $($fields: self.$fields - rhs.$fields,)*
                }
            }
        }

        impl std::ops::SubAssign for $type_name {
            fn sub_assign(&mut self, rhs: Self) {
                *self = $type_name {
                    $($fields: self.$fields - rhs.$fields,)*
                }
            }
        }

        // Implement scalar multiplication.
        impl std::ops::Mul<f64> for $type_name {
            type Output = Self;
    
            fn mul(self, rhs: f64) -> Self::Output {
                $type_name {
                    $($fields: self.$fields * rhs,)*
                }
            }
        }
    
        impl std::ops::Mul<$type_name> for f64 {
            type Output = $type_name;
    
            fn mul(self, rhs: $type_name) -> Self::Output {
                rhs * self
            }
        }
    
        impl std::ops::MulAssign<f64> for $type_name {
            fn mul_assign(&mut self, rhs: f64) {
                *self = $type_name {
                    $($fields: self.$fields * rhs,)*
                }
            }
        }

        // Implement scalar division.
        impl std::ops::Div<f64> for $type_name {
            type Output = $type_name;
    
            fn div(self, rhs: f64) -> Self::Output {
                $type_name {
                    $($fields: self.$fields / rhs,)*
                }
            }
        }
    
        impl std::ops::DivAssign<f64> for $type_name {
            fn div_assign(&mut self, rhs: f64) {
                *self = $type_name {
                    $($fields: self.$fields / rhs,)*
                }
            }
        }
    };
}