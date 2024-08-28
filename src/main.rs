fn main() {
    let width: i32 = 256;
    let height: i32 = 256;

    print!("P3\n{width} {height}\n255\n");

    for j in 0..height {
        eprintln!("Lines remaining: {}", height - j);
        for i in 0..width {
            let r: f64 = i as f64 / (width - 1) as f64;
            let g: f64 = j as f64 / (height - 1) as f64;
            let b: f64 = 0.0;

            let ir: i64 = (255.999 * r) as i64;
            let ig: i64 = (255.999 * g) as i64;
            let ib: i64 = (255.999 * b) as i64;

            println!("{ir} {ig} {ib}");
        }
    }
}
