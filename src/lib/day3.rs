use std::cmp;
use std::collections::{ HashMap, HashSet};
use itertools::Itertools;
use std::string::String;

struct Dimension {
    x: i32,
    y: i32,
}

struct Rectangle {
    id: i32, 
    start: Dimension,
    end: Dimension
}

impl Dimension {
    fn new(x: i32, y: i32) -> Dimension {
        Dimension {
            x: x,
            y: y,
        } 
    }
}

fn get_start_from_token(token: &str) -> Dimension {
    let pre_colon = token.split(":").next().unwrap();
    let dim_strs: Vec<i32> = pre_colon.split(",").map(|i| i.parse::<i32>().unwrap()).collect();
    Dimension::new(dim_strs[0], dim_strs[1])
}

fn get_size_from_token(token: &str) -> Dimension {
    let dim_strs: Vec<i32> = token.split("x").map(|i| i.parse::<i32>().unwrap()).collect();
    Dimension::new(dim_strs[0], dim_strs[1])
}

fn parse_line(line: &str) -> (i32, Dimension, Dimension) {
    let parts: Vec<&str> = line.split(" ").collect();
    let id = parts[0].split_at(1).1.parse::<i32>().unwrap();
    let start = get_start_from_token(parts[2]);
    let size = get_size_from_token(parts[3]);  
    (id, start, size)
}

impl Rectangle {
    fn new_size(id: i32, start: Dimension, size: Dimension) -> Rectangle {
        let end = Dimension {
            x: start.x + size.x,
            y: start.y + size.y,
        };
        Rectangle {
            id: id,
            start: start,
            end: end,
        } 
    }

    fn new_end(id: i32, start: Dimension, end: Dimension) -> Rectangle {
        Rectangle {
            id: id,
            start: start,
            end: end,
        }
    }

    fn get_overlap(&self, rect: &Rectangle) -> Option<Rectangle> {
        if self.start.x > rect.end.x || rect.start.x > self.end.x {
            return None
        } else if self.start.y > rect.end.y || rect.start.y > self.end.y {
            return None
        }
        let overlap_start = Dimension {
            x: cmp::max(self.start.x, rect.start.x),
            y: cmp::max(self.start.y, rect.start.y),
        };
        let overlap_end = Dimension {
            x: cmp::min(self.end.x, rect.end.x),
            y: cmp::min(self.end.y, rect.end.y),
        };
        Some(Rectangle::new_end(0, overlap_start, overlap_end))
    }
}

fn get_overlap_volumes(overlaps: Vec<Rectangle>) -> i32 {
    let mut row_counts: HashMap<i32, HashMap<i32, bool>> = HashMap::new();
    for overlap in overlaps.iter() {
        for row in overlap.start.x..overlap.end.x {
            let row_map = row_counts.entry(row).or_insert(HashMap::new());
            for col in overlap.start.y..overlap.end.y {
                row_map.insert(col, true);
            }
        }
    }
    row_counts.values().map(|hm: &HashMap<i32, bool>| hm.len() as i32).sum()
}

fn get_list_of_rectangles(input: &str) -> Vec<Rectangle> {
    let mut rects: Vec<Rectangle> = Vec::new();
    for line in input.lines() {
        let dims = parse_line(line);
        let rect = Rectangle::new_size(dims.0, dims.1, dims.2);
        rects.push(rect);
    }
    rects
}

pub fn process_input(input: &str) -> (i32, i32) {
    let rects = get_list_of_rectangles(input);
    let mut overlaps: Vec<Rectangle> = Vec::new();
    let mut intacts: HashSet<i32> = rects.iter().map(|rect| rect.id).collect(); 
    for comb in rects.iter().combinations(2) {
        if let Some(overlap) = comb[0].get_overlap(comb[1].clone()) {
            overlaps.push(overlap);
            intacts.remove(&comb[0].id);
            intacts.remove(&comb[1].id);
        }
    }
    (get_overlap_volumes(overlaps), *intacts.iter().next().unwrap())
}
