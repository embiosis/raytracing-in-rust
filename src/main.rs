extern crate bmp;

#[allow(unused_imports)]
use bmp::{Image, Pixel, px};
use raytracer::graphics::{viewport::*, vec3::*};

use std::{env::args, time::{SystemTime, UNIX_EPOCH}};

const DEFAULT_WIDTH: u32 = 1920;
const DEFAULT_HEIGHT: u32 = 1080;

// TODO: Remove this enum and the attribute later.
#[allow(dead_code)]
enum FileNameType {
    UnixTime,
    Dimensions
}

fn main() {
    let args: Vec<String> = args().collect();
    let mut width: u32 = DEFAULT_WIDTH;
    let mut height: u32 = DEFAULT_HEIGHT;

    // parse arguments
    if args.len() >= 3 {
        width = args[2].trim().parse().unwrap();
    } else if args.len() >= 4 {
        height = args[3].trim().parse().unwrap();
    } else {
        println!("Usage: cargo run -- [test | render] [width] [aspect-ratio]");
        return;
    }

    let viewport = Viewport::new(width, height, 0.0, 2.0, Vec3::zero());

    // TODO: Remove this stub and use clap to properly parse arguments!
    // TODO: Take a user-defined file name, or use the current time as a default.
    let file_name_type = FileNameType::UnixTime;

    let save_path = match file_name_type {
        FileNameType::UnixTime => format!("output/test-{}.bmp", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()),
        FileNameType::Dimensions => format!("output/test-{}x{}.bmp", viewport.image_width, viewport.image_height),
    };
    
    match args.get(1).unwrap().trim() {
        "test" => viewport.test(save_path),
        "render" => viewport.render(save_path),
        _ => eprintln!("Unexpected raytracing mode. Please specify either test or render.")
    }
}
