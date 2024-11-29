use super::{colour::Colour, sphere::Sphere, vec3::*};
use bmp::Pixel;

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
        if Sphere::new(0, 0, -1, 0.5, None).hit(self) {
            // TODO: Remove placeholder colour
            Colour::red().into()
        } else {
            self.gradient(Colour::white(), Colour::new(128, 179, 255))
        }
    }

    // TODO: Add functionality to specify the direction of the gradient
    pub fn gradient(&self, start_colour: Colour, end_colour: Colour) -> Pixel {
        let unit_vec = self.direction.normalise();
        let y = 0.5 * (unit_vec.y + 1.0);

        // ! DEBUG STATEMENT
        // println!("{y}");
        ((1.0 - y) * start_colour + y * end_colour).into()
    }

    pub fn center_viewport(&self) -> Pixel {
        let Vec3 {x, y, ..} = self.direction.normalise();
        
        if y <= 0.05 && y >= -0.05 {
            Colour::magenta().into()
        } else if x <= 0.05 && x >= -0.05 {
            Colour::magenta().into()
        } else {
            Colour::white().into()
        }
    }
}