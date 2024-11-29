use super::colour::Colour;

#[derive(Clone, Default)]
pub struct Material {
    colour: Colour,
    // albedo: f64,
    // metallic: f64,
    // texture: Option<String>,
}

impl Material {
    pub fn new(colour: Colour) -> Self {
        Material {
            colour,
        }
    }
}