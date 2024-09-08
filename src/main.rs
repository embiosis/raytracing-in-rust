extern crate bmp;

use bmp::{Image, Pixel};
use raytracer::vec3::Vec3;


fn main() {
    let width: i32 = 256;
    let height: i32 = 256;

    let x = Vec3(1.0, 2.0, 3.0);
    let y = Vec3(2.0, 2.0, 3.0);

    let z = x * y;
    let b = x - y;
    let f = 5 as f64 * y;
    let g = y * 3.5;

    x.print();
    y.print();
    println!("{z}");

    print!("P3\n{width} {height}\n255\n");

    // for j in 0..height {
    //     eprintln!("Lines remaining: {}", height - j);
    //     for i in 0..width {
    //         let colour = Vec3(i as f64 / (width - 1) as f64, j as f64 / (height - 1) as f64, 0.0);
    //     }
    // }
    // eprintln!("Completed!");
}
