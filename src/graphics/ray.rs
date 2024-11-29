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

    pub fn hit_sphere(&self, sphere: &Sphere) -> Option<f64> {
        let position_vec = sphere.centre - self.origin;
        
        // Use the quadratic formula to check for intersections
        // between the ray and the sphere.
        let a = self.direction * self.direction;
        let h = self.direction * position_vec;
        let c = position_vec * position_vec - sphere.radius * sphere.radius;

        let discriminant = h * h - a * c;
        
        if discriminant < 0.0 {
            None
        } else {
            Some( (h - discriminant.sqrt()) / a )
        }
    }

    // TODO: Add functionality (currently a stub)
    pub fn get_colour(&self) -> Pixel {
        let sphere = Sphere::new(0, 0, -1, 0.5, None);

        if let Some(lambda) = self.hit_sphere(&sphere) {
            let normal = sphere
                .normal(self.at(lambda))
                .expect("The point should always lie on the sphere");

            (0.5 * Colour::lerp(1.0 + normal.x, 1.0 + normal.y, 1.0 + normal.z)).into()
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