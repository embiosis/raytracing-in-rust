extern crate bmp;

#[allow(unused_imports)]
use bmp::{Image, Pixel, px};
use raytracer::graphics::{viewport::*, vec3::*};

use std::{env::args, time::{SystemTime, UNIX_EPOCH}};

const DEFAULT_WIDTH: u32 = 1920;
const DEFAULT_ASPECT_RATIO: f64 = 16.0 / 9.0;

enum FileNameType {
    UnixTime,
    Dimensions
}

fn main() {
    let args: Vec<String> = args().collect();
    let mut width: u32 = DEFAULT_WIDTH;
    let mut aspect_ratio: f64 = DEFAULT_ASPECT_RATIO;

    // parse arguments
    if args.len() >= 2 {
        width = args[1].trim().parse().unwrap();
    } else if args.len() >= 3 {
        aspect_ratio = args[2].trim().parse().unwrap();
    } else {
        println!("Usage: cargo run -- [width] [aspect-ratio]");
        return;
    }

    let viewport = Viewport::new(width, 0.0, 2.0, aspect_ratio, Vec3::zero());

    // TODO: Remove this stub and use clap to properly parse arguments!
    let file_name_type = FileNameType::UnixTime;

    let save_path = match file_name_type {
        FileNameType::UnixTime => format!("output/test-{}.bmp", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()),
        FileNameType::Dimensions => format!("output/test-{}x{}.bmp", viewport.image_width, viewport.image_height),
    };
    
    viewport.render(save_path);
}
