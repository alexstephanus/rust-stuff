#[allow(unused_imports)]

use std::collections::HashMap;
use std::fs;

pub fn day_2() -> () {
    let input = fs::read_to_string("src/inputs/2.txt")
        .expect("Not a file");
    let res = get_letter_counts(&input);
    println!("{}", res);
}

fn get_letter_counts(box_id: &str) -> HashMap<char, i32> {
    let mut letter_counts: HashMap<char, i32> = HashMap::new();
    for c in box_id.chars() {
        let counter = letter_counts.entry(c).or_insert(0);
        counter += 1;
    };
    letter_counts
}

fn find_checksum(box_ids: &str) -> i32 {
    let mut two_letters = 0;
    let mut three_letters = 0;
    for box_id in box_ids.lines() {
        let distinct_counts: Vec<i32> = get_letter_counts(box_id).values().collect();
        if distinct_counts.contains(3) { three_letters++; };
        if distinct_counts.contains(2) { two_letters++; };
    };
    three_letters * two_letters
}