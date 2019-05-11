use std::cmp;

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x, y) {
        Point {
            x: x,
            y: y
        }
    }

    pub fn dist(self, other: &Point) {
        let x_dist = self.x - other.x;
        let y_dist = self.y - other.y;
        x_dist.abs() + y_dist.abs()
    }
}

fn find_closest_points(point: Point, refs: &Vec<Point>) -> Vec<Point> {
    
}

fn get_closest_lists_for_squares(point, refs) 
    

fn find_infinite_areas(points: Vec<Point>) -> Vec<Point> {
    let mut min_x: i32 = points[0].x;
    let mut max_x: i32 = points[0].x;
    let mut min_y: i32 = points[0].y;
    let mut max_y: i32 = points[0].y;
    for p in points.iter() {
        min_x = cmp::min(min_x, p.x);
        max_x = cmp::max(max_x, p.x);
        min_y = cmp::min(min_y, p.y);
        max_y = cmp::max(max_y, p.y);
    }
     
}
