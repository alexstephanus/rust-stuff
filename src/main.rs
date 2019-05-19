#[allow(unused_imports)]

mod lib;

use lib::{ aoc_utils, day1, day2, day3, day5, day10 };
use std::collections::HashMap;

fn insert_two(hm: &mut HashMap<i32, i32>) {
    hm.insert(0,1);
    hm.insert(1,0);
} 

fn main() {
    println!("Hello, world!");
    day1::day_1();
    day2::day_2();
    let day3_input = aoc_utils::get_day_input(3);
    let res3 = day3::process_input(&day3_input);
    let day5_input = aoc_utils::get_day_input(5);
    println!("{}, {}", res3.0, res3.1);
    println!(
        "Day 5: {}, {}",
        day5::polymer_reaction(&day5_input, None),
        day5::optimal_removal(&day5_input)
    );
    let mut hm = HashMap::new();
    insert_two(&mut hm);
    let day10_input = aoc_utils::get_day_input(10);
    let(msg, steps) = day10::get_message(&day10_input);
    println!("{}, {}", msg, steps);
}

