extern crate bmp;

#[allow(unused_imports)]
use bmp::{Image, Pixel, px};
use raytracer::graphics::{viewport::*, vec3::*};

use std::{env::args, time::{SystemTime, UNIX_EPOCH}};

const DEFAULT_WIDTH: u32 = 1920;
const DEFAULT_ASPECT_RATIO: f64 = 16.0 / 9.0;

// TODO: Remove this enum and the attribute later.
#[allow(dead_code)]
enum FileNameType {
    UnixTime,
    Dimensions
}

fn main() {
    let args: Vec<String> = args().collect();
    let mut width: u32 = DEFAULT_WIDTH;
    let height: u32;

    // parse arguments
    if args.len() < 3 {
        println!("usage: cargo run -- [test | render] [width] (height)");
        println!("if no height is provided, the program will default to an aspect ratio of 16:9");
        return;
    }

    if args.len() >= 3 {
        width = args[2].trim().parse().unwrap();
    }
    
    if args.len() >= 4 {
        height = args[3].trim().parse().unwrap();
    } else {
        height = (width as f64 / DEFAULT_ASPECT_RATIO) as u32;
    }

    let viewport = Viewport::new(width, height, 1.0, 2.0, Vec3::zero());

    // TODO: Remove this stub and use clap to properly parse arguments!
    // TODO: Take a user-defined file name, or use the current time as a default.
    let file_name_type = FileNameType::UnixTime;

    let save_path = match file_name_type {
        FileNameType::UnixTime => format!("output/test-{}.bmp", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()),
        FileNameType::Dimensions => format!("output/test-{}x{}.bmp", viewport.image_width, viewport.image_height),
    };
    
    match args.get(1).unwrap().trim() {
        "test" => viewport.test_file_format(save_path),
        "render" => viewport.test_gradient(save_path),
        _ => eprintln!("Unexpected raytracing mode. Please specify either test or render.")
    }
}
