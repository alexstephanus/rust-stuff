use std::cmp;

#[derive(Debug, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
    v_x: i32,
    v_y: i32,
}

impl Coord {
    fn new(x: i32, y: i32, v_x: i32, v_y: i32) -> Coord {
        Coord { x, y, v_x, v_y }
    }

    fn step(&mut self, steps: i32) {
        self.x += self.v_x * steps;
        self.y += self.v_y * steps;
    }

    fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

fn get_grid_specs(points: &Vec<Coord>) -> (i32, i32, i32, i32) {
    let (first_x, first_y) = points.get(0).unwrap().pos();
    let mut min_x = first_x;
    let mut max_x = first_x;
    let mut min_y = first_y;
    let mut max_y = first_y;

    for point in points.iter() {
        let (x, y) = point.pos();
        min_x = cmp::min(x, min_x);
        min_y = cmp::min(y, min_y);
        max_x = cmp::max(x, max_x);
        max_y = cmp::max(y, max_y);
    }
    (max_x - min_x + 1, max_y - min_y + 1, min_x, min_y)
}

fn get_grid_size(grid: &Vec<Coord>) -> i32 {
    let (x, y, _, _) = get_grid_specs(grid);
    x + y
}

fn replace_index(line: &str, index: usize) -> String {
    line.chars().enumerate()
        .map(|(i, c)| if i == index {'#'} else {c})
        .collect()
}

fn get_grid_display(grid: &Vec<Coord>) -> String {
    let (w, h, min_x, min_y) = get_grid_specs(grid);
    let mut lines: Vec<String> = Vec::new();
    for _ in 0..h {
        lines.push(".".repeat(w as usize));
    }
    for p in grid.iter() {
        let (x, y) = p.pos();
        let vec_x = x - min_x;
        let vec_y = y - min_y;
        let row = lines.get_mut(vec_y as usize).unwrap();
        *row = replace_index(&row, vec_x as usize);
    }
    lines.join("\n")
}

fn step_grid(grid: &mut Vec<Coord>, steps: i32, step_count: &mut i32) {
    for i in grid.iter_mut() {
        i.step(steps);
    }
    *step_count += steps;
}

fn find_smallest_grid(grid: &mut Vec<Coord>) -> (String, i32) {
    let mut step_size = 1024;
    let mut cur_size = get_grid_size(grid);
    let mut steps = 0;
    loop {
        step_grid(grid, step_size, &mut steps);
        let new_size = get_grid_size(grid);
        if new_size < cur_size {
            cur_size = new_size;
        }else {
            if step_size == 1 {
                step_grid(grid, -1, &mut steps);
                break;
            } else {
                step_grid(grid, -2 * step_size, &mut steps);
                step_size /= 2;
                cur_size = get_grid_size(grid);
            }
        }
    }
    (get_grid_display(grid), steps)
}

fn unwrap_bracket(bracket: &str) -> (i32, i32) {
    let nums1 = bracket.replace(|c: char| c == '<' || c == '>' || c.is_whitespace(), "");
    let mut nums = nums1.split(',');
    let x = nums.next().unwrap().parse::<i32>().unwrap();
    let y = nums.next().unwrap().parse::<i32>().unwrap();
    (x, y)
}

fn parse_line(line: &str) -> Coord {
   let brackets1 = line.replace(|c| "positn=velcy".contains(c), "");
   let mut brackets = brackets1.split("> <");
   let first_bracket = brackets.next().unwrap();
   let second_bracket = brackets.next().unwrap();
   let (x,y) = unwrap_bracket(first_bracket);
   let (v_x, v_y) = unwrap_bracket(second_bracket);
   Coord::new(x, y, v_x, v_y)
}

fn parse_input(input: &str) -> Vec<Coord>{
    input.lines().map(|line| parse_line(line)).collect()     
}

pub fn get_message(input: &str) -> (String, i32) {
    find_smallest_grid(&mut parse_input(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let sample = "position=< 9, 1> velocity=< 0, 2>";
        let c = parse_line(&sample);
        assert_eq!(c, Coord::new(9, 1, 0, 2));
    }

    #[test]
    fn test_parse_input() {
        let sample_input = "position=< 9, 1> velocity=< 0, 2>
position=<5, 4> velocity=<0, 0>";
        let c = parse_input(&sample_input);
        assert_eq!(c, vec![Coord::new(9, 1, 0, 2), Coord::new(5, 4, 0, 0)]);
        let size = get_grid_specs(&c);
        assert_eq!(size, (5, 4, 5, 1));
    }
    
    #[test]
    fn test_grid_display() {
        let sample_input = "position=< 9, 1> velocity=< 0, 2>
position=<5, 4> velocity=<0, 0>";
        let c = parse_input(&sample_input);
        let expected_grid = "....#
.....
.....
#....";
        assert_eq!(expected_grid, get_grid_display(&c));
    }

    #[test]
    fn test_get_message() {
        let sample_input = "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";
        let expected_output = "#...#..###
#...#...#.
#...#...#.
#####...#.
#...#...#.
#...#...#.
#...#...#.
#...#..###";
        let (output, steps) = get_message(&sample_input);
        assert_eq!(output, expected_output);
        assert_eq!(steps, 3);
    }
}
