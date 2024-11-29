extern crate bmp;

#[allow(unused_imports)]
use bmp::{Image, Pixel, px};
use raytracer::graphics::{viewport::*, vec3::*};

use std::env::args;

const DEFAULT_WIDTH: u32 = 1920;
const DEFAULT_ASPECT_RATIO: f64 = 16.0 / 9.0;

fn main() {
    let args: Vec<String> = args().collect();
    let mut width: u32 = DEFAULT_WIDTH;
    let mut aspect_ratio: f64 = DEFAULT_ASPECT_RATIO;

    // parse arguments
    if args.len() >= 2 {
        width = args[1].trim().parse().unwrap();
    }

    if args.len() >= 3 {
        aspect_ratio = args[2].trim().parse().unwrap();
    }


    let viewport = Viewport::new(width, 0.0, 2.0, aspect_ratio, Vec3::zero());

    viewport.render(format!("output/test-{}x{}.bmp", viewport.image_width, viewport.image_height));
}
