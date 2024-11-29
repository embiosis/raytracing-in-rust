use super::{colour::Colour, vec3::*};
use bmp::{Pixel, px};

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

    // TODO: Add functionality to specify the direction of the gradient
    pub fn gradient(&self, start_colour: Colour, end_colour: Colour) -> Pixel {
        let unit_vec = self.direction.normalise();
        let y = 0.5 * (unit_vec.y + 1.0);
        ((1.0 - y) * start_colour + y * end_colour).into()
    }
}