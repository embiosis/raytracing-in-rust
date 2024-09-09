extern crate bmp;

use bmp::{Image, Pixel, px};
use raytracer::vec3::Vec3;
use raytracer::ray::Ray;


fn main() {
    let aspect_ratio: f64 = (16 as f64) / (9 as f64);
    println!("{aspect_ratio}");

    let width: u32 = 400;
    let height: u32 = std::cmp::max((width as f64 / aspect_ratio) as u32, 1);

    let z = Ray {origin: Vec3(1.0, 2.0, 3.0), direction: Vec3(5.0, 5.5, 3.0)};


    let mut img = Image::new(width, height);

    for (x, y) in img.coordinates() {
        if x == 0 {
            // eprintln!("Scanlines remaining: {}", height - y);
        }
        img.set_pixel(x, y, px!(255.999 * x as f64 / (width - 1) as f64, 255.999 * y as f64 / (height - 1) as f64, 255.999 * 0.0));
    }

    // eprintln!("Completed!");

    let _ = img.save("output/test.bmp");
    // eprintln!("File saved!");
}
