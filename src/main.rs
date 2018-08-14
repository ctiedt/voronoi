extern crate image;
extern crate rand;

use std::env;
use std::path::Path;
use rand::random;

fn dist(p1: [f64; 2], p2: [f64; 2]) -> f64{
    ((p1[0] - p2[0]).powf(2.0) + (p1[1] - p2[1]).powf(2.0)).sqrt()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {panic!("Expected at least 4 arguments, got {} arguments", args.len() - 1)}
    let size_x = args[1].parse::<u32>().unwrap();
    let size_y = args[2].parse::<u32>().unwrap();
    let n_points = args[3].parse::<usize>().unwrap();
    let mut points: Vec<[f64; 2]> = vec![];
    let mut colours: Vec<[u8; 3]> = vec![];
    for _ in 0..n_points {
        points.push([(random::<u32>() % size_x) as f64, (random::<u32>() % size_y) as f64]);
        colours.push([random::<u8>(), random::<u8>(), random::<u8>()])
    }
    let mut img: image::RgbImage = image::ImageBuffer::new(size_x, size_y);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let mut min = (size_x * size_y) as f64;
        let mut min_i = 0;
        for i in 0..n_points {
            if dist([x as f64, y as f64], points[i]) < min {
                min = dist([x as f64, y as f64], points[i]);
                min_i = i;
            }
        }
        *pixel = image::Rgb::<u8> {data: colours[min_i]};
    }
    let p = Path::new("./out.png");
    img.save(p);
}
