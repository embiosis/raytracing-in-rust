#[allow(dead_code)]
#[allow(unused)]

use raytracer::vec3::Vec3;

fn main() {
    let width: i32 = 256;
    let height: i32 = 256;

    let x = Vec3(1.0, 2.0, 3.0);

    print!("P3\n{width} {height}\n255\n");

    for j in 0..height {
        eprintln!("Lines remaining: {}", height - j);
        for i in 0..width {
            let colour = Vec3(i as f64 / (width - 1) as f64, j as f64 / (height - 1) as f64, 0.0);
        }
    }
    eprintln!("Completed!");
}
