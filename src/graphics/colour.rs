use std::fmt::Display;
use crate::impl_vec_struct;
use bmp::{Pixel, px};

// ! Remove this if unused
// const RGB_SCALE: f64 = 255.999;

impl_vec_struct!(pub Colour => {r: f64, g: f64, b: f64});

impl Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Colour({}, {}, {})", self.r, self.g, self.b)
    }
}

impl Colour {
    // Check whether the colour is a valid RGB value.
    fn is_valid(&self) -> bool {
        (self.r >= 0.0 && self.r <= 255.0) && 
        (self.g >= 0.0 && self.g <= 255.0) && 
        (self.b >= 0.0 && self.b <= 255.0)
    }

    // Create a new colour from any types that can be casted into f64.
    pub fn new<T>(r: T, g: T, b: T) -> Self 
    where
        T: Into<f64>
    {
        Self {
            r: r.into(),
            g: g.into(),
            b: b.into(),
        }
    }

    // Functions to initialise some basic RGB colours.
    pub fn black() -> Self {
        Self::new(0, 0, 0)
    }

    pub fn white() -> Self {
        Self::new(255, 255, 255)
    }

    pub fn red() -> Self {
        Self::new(255, 0, 0)
    }

    pub fn green() -> Self {
        Self::new(0, 255, 0)
    }
    
    pub fn blue() -> Self {
        Self::new(0, 0, 255)
    }
    
    pub fn yellow() -> Self {
        Self::new(255, 255, 0)
    }
    
    pub fn cyan() -> Self {
        Self::new(0, 255, 255)
    }
    
    pub fn magenta() -> Self {
        Self::new(255, 0, 255)
    }
}

impl From<Colour> for Pixel {
    // ? Should we check the validity of the Colour before conversion?
    fn from(value: Colour) -> Self {
        px!(value.r as u8, value.g as u8, value.b as u8)
    }
}

    