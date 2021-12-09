use std::time::Instant;
use array2d::Array2D;
use image::{GenericImageView, Pixel};

use self::maze::solver::Point;

mod maze;

fn main() {
    let img = image::open("maze1000.png").unwrap();
    let width = img.width() as usize;
    let height = img.height() as usize;
    println!("Loaded image: {} x {}", width, height);
    if width <= 2 || height <= 2 {
        eprintln!("Image size must be at least 3x3");
        return;
    }
    // Process image into maze boolean array
    println!("Transforming image into maze...");
    let now = Instant::now();
    let mut maze = Array2D::filled_with(false, img.width() as usize, img.height() as usize);
    for (x, y, color) in img.pixels() {
        let rgb = color.to_rgb().0;
        let filled = rgb[0] > 0 || rgb[1] > 0 || rgb[2] > 0;
        maze.set(x as usize, y as usize, filled).unwrap();
    }
    println!("Transformed image into maze in {}ns", now.elapsed().as_nanos());
    let start: Point = Point {
        x: 0,
        y: 1,
    };
    let end = Point {
        x: 2000,
        y: 1999,
    };
    println!("Solving maze from ({},{}) to ({},{}) ...", start.x, start.y, end.x, end.y);
    // Test "cold"
    let now = Instant::now();
    let mut success = maze::solver::is_reachable(&maze, &start, &end);
    let cold_time = now.elapsed().as_nanos();
    // Warm up
    for _ in 0..1000 {
        success = maze::solver::is_reachable(&maze, &start, &end);
    }
    // Test "warm"
    let now = Instant::now();
    success = maze::solver::is_reachable(&maze, &start, &end);
    let warm_time = now.elapsed().as_nanos();
    if success {
        println!("Solved maze in: cold={}ns, warm={}ns", cold_time, warm_time);
    } else {
        eprintln!("Failed to solve maze.");
    }
}
