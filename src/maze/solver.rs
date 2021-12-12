use super::array2d::Array2D;

#[derive(Copy, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub fn is_reachable(maze: &Array2D<bool>, start: &Point, end: &Point) -> bool {
    let width = maze.width;
    let width_m1 = width - 1;
    let height = maze.height;
    let height_m1 = height - 1;

    let mut visited = Array2D::new(false, width, height);
    visited.set(start.x, start.y, true);

    let mut stack: Vec<Point> = vec![];
    let mut p: Point = *start;
    loop {
        let x = p.x;
        let y = p.y;

        // Check if we've reached the end
        if x == end.x && y == end.y {
            return true;
        }

        // Check left
        if x > 0 {
            let left = x - 1;
            if !visited.get(left, y) && maze.get(left, y) {
                // Go left
                stack.push(Point {
                    x: left,
                    y,
                });
                visited.set(left, y, true);
            }
        }
        // Check right
        if x < width_m1 {
            let right = x + 1;
            if !visited.get(right, y) && maze.get(right, y) {
                // Go right
                stack.push(Point {
                    x: right,
                    y,
                });
                visited.set(right, y, true);
            }
        }
        // Check up
        if y > 0 {
            let up = y - 1;
            if !visited.get(x, up) && maze.get(x, up) {
                // Go up
                stack.push(Point {
                    x,
                    y: up,
                });
                visited.set(x, up, true);
            }
        }
        // Check down
        if y < height_m1 {
            let down = y + 1;
            if !visited.get(x, down) && maze.get(x, down) {
                // Go down
                stack.push(Point {
                    x,
                    y: down,
                });
                visited.set(x, down, true);
            }
        }

        match stack.pop() {
            Some(next) => {
                p = next;
            },
            None => {
                break;
            }
        }
    }
    return false;
}
