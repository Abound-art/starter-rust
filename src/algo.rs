use image::{ImageBuffer, Rgb, RgbImage};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    beta: f64,
    rho: f64,
    sigma: f64,
    dt: f64,
    iterations: i32,
    result_size: i32,
}

#[derive(Clone, Copy)]
struct Point(f64, f64, f64);

struct Bounds {
    min: Point,
    max: Point,
}

impl Bounds {
    fn expand(&mut self, p: &Point) {
        if p.0 < self.min.0 {
            self.min.0 = p.0
        }
        if p.1 < self.min.1 {
            self.min.1 = p.1
        }
        if p.2 < self.min.2 {
            self.min.2 = p.2
        }
        if p.0 > self.max.0 {
            self.max.0 = p.0
        }
        if p.1 > self.max.1 {
            self.max.1 = p.1
        }
        if p.2 > self.max.2 {
            self.max.2 = p.2
        }
    }

    fn translate(&self, p: &Point, result_size: i32) -> Point {
        let rel_x = (p.0 - self.min.0) / (self.max.0 - self.min.0);
        let rel_y = (p.1 - self.min.1) / (self.max.1 - self.min.1);
        let rel_z = (p.2 - self.min.2) / (self.max.2 - self.min.2);
        let s = (result_size - 1) as f64;
        Point(rel_x * s, rel_y * s, rel_z * s)
    }
}

pub fn run(c: Config) -> RgbImage {
    let mut points = vec![Point(0.0, 0.0, 0.0); c.iterations as usize];
    points[0] = Point(1.0, 1.0, 1.0);
    let mut bounds = Bounds {
        min: Point(1.0, 1.0, 1.0),
        max: Point(1.0, 1.0, 1.0),
    };
    for i in 1..c.iterations {
        let p = next_step(&c, &points[(i - 1) as usize]);
        points[i as usize] = p;
        bounds.expand(&p);
    }
    for p in points.iter_mut() {
        *p = bounds.translate(p, c.result_size);
    }
    let mut counts: Vec<Vec<i32>> = vec![vec![0; c.result_size as usize]; c.result_size as usize];

    let mut max_count = 0i32;
    for p in points {
        let x = p.0.floor() as usize;
        let y = p.1.floor() as usize;
        counts[x][y] += 1;
        if counts[x][y] > max_count {
            max_count = counts[x][y];
        }
    }

    let mut img: RgbImage = ImageBuffer::new(c.result_size as u32, c.result_size as u32);

    for (i, row) in counts.iter().enumerate() {
        for (j, count) in row.iter().enumerate() {
            if *count == 0 {
                img.put_pixel(i as u32, j as u32, Rgb([0, 0, 0]));
            } else {
                let pos = ((*count as f64) / (max_count as f64)).sqrt().sqrt();
                let b = (pos * 200.0) as u8 + 55;
                let g = ((1.0 - pos) * 200.0) as u8 + 55;
                img.put_pixel(i as u32, j as u32, Rgb([0, g, b]));
            }
        }
    }

    img
}

fn next_step(c: &Config, p: &Point) -> Point {
    let dxdt = c.sigma * (p.1 - p.0);
    let dydt = p.0 * (c.rho - p.2) - p.1;
    let dzdt = p.0 * p.1 - c.beta * p.2;
    Point(p.0 + dxdt * c.dt, p.1 + dydt * c.dt, p.2 + dzdt * c.dt)
}
