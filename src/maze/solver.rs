use array2d::Array2D;

#[derive(Copy, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub fn is_reachable(maze: &Array2D<bool>, start: &Point, end: &Point) -> bool {
    let width = maze.num_rows();
    let width_m1 = width - 1;
    let height = maze.num_columns();
    let height_m1 = height - 1;

    let mut visited = Array2D::filled_with(false, width, height);
    visited[(start.x, start.y)] = true;

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
            if !visited[(left, y)] && maze[(left, y)] {
                // Go left
                stack.push(Point {
                    x: left,
                    y,
                });
                visited[(left, y)] = true;
            }
        }
        // Check right
        if x < width_m1 {
            let right = x + 1;
            if !visited[(right, y)] && maze[(right, y)] {
                // Go right
                stack.push(Point {
                    x: right,
                    y,
                });
                visited[(right, y)] = true;
            }
        }
        // Check up
        if y > 0 {
            let up = y - 1;
            if !visited[(x, up)] && maze[(x, up)] {
                // Go up
                stack.push(Point {
                    x,
                    y: up,
                });
                visited[(x, up)] = true;
            }
        }
        // Check down
        if y < height_m1 {
            let down = y + 1;
            if !visited[(x, down)] && maze[(x, down)] {
                // Go down
                stack.push(Point {
                    x,
                    y: down,
                });
                visited[(x, down)] = true;
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
