extern crate image;
extern crate rand;

use rand::random;
use std::env;
use std::path::Path;

#[derive(Copy, Clone)]
struct RGBColour(u8, u8, u8);

impl RGBColour {
    fn as_pixel(self) -> image::Rgb<u8> {
        image::Rgb::<u8> {
            data: [self.0, self.1, self.2],
        }
    }

    fn random() -> Self {
        Self(random::<u8>(), random::<u8>(), random::<u8>())
    }
}

#[derive(Copy, Clone)]
struct Point(f64, f64);

impl Point {
    fn distance_to(self, x: f64, y: f64) -> f64 {
        ((self.0 as f64 - x).powf(2.0) + (self.1 as f64 - y).powf(2.0)).sqrt()
    }

    fn random(size_x: u32, size_y: u32) -> Self {
        Self(
            (random::<u32>() % size_x) as f64,
            (random::<u32>() % size_y) as f64,
        )
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        panic!(
            "Expected at least 4 arguments, got {} arguments",
            args.len() - 1
        )
    }
    let size_x = args[1].parse::<u32>().unwrap();
    let size_y = args[2].parse::<u32>().unwrap();
    let n_points = args[3].parse::<usize>().unwrap();
    let mut points: Vec<Point> = vec![];
    let mut colours: Vec<RGBColour> = vec![];
    for _ in 0..n_points {
        points.push(Point::random(size_x, size_y));
        colours.push(RGBColour::random())
    }
    let mut img: image::RgbImage = image::ImageBuffer::new(size_x, size_y);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let mut min = (size_x * size_y) as f64;
        let mut min_i = 0;
        for i in 0..n_points {
            if points[i].distance_to(x as f64, y as f64) < min {
                min = points[i].distance_to(x as f64, y as f64);
                min_i = i;
            }
        }
        *pixel = colours[min_i].as_pixel();
    }
    let p = Path::new(&args[4]);
    let _ = img.save(p);
}
