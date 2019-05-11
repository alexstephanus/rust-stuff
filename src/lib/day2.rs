#[allow(unused_imports)]

use std::collections::HashMap;
use std::fs;
use std::string::String;

pub fn day_2() -> () {
    let input = fs::read_to_string("lib/inputs/2.txt")
        .expect("Not a file");
    let res = find_checksum(&input);
    println!("{}", res);
    let res2 = find_close_strings(&input);
    println!("{}", res2);
}

fn get_letter_counts(box_id: &str) -> HashMap<char, i32> {
    let mut letter_counts: HashMap<char, i32> = HashMap::new();
    for c in box_id.chars() {
        let counter = letter_counts.entry(c).or_insert(0);
        *counter += 1;
    };
    letter_counts
}

fn find_checksum(box_ids: &str) -> i32 {
    let mut two_letters = 0;
    let mut three_letters = 0;
    for box_id in box_ids.lines() {
        let distinct_counts: Vec<i32> = get_letter_counts(box_id)
            .values()
            .map(|val| *val) 
            .collect::<Vec<i32>>();
        if distinct_counts.contains(&3) {
            three_letters += 1;
        };
        if distinct_counts.contains(&2) {
            two_letters += 1;
        };
    };
    three_letters * two_letters
}

fn find_string_distance(id_1: &str, id_2: &str) -> (i32, String) {
    let iter_1 = id_1.chars();
    let mut iter_2 = id_2.chars();
    let mut dist = 0;
    let mut same = String::new();
    for char_1 in iter_1 {
        if char_1 != iter_2.next().expect("Are these two differently-sized strings?") {
            dist += 1;
        } else {
            same.push(char_1);
        }
    }
    (dist, same)
}

fn find_close_strings(box_ids: &str) -> String {
    let mut ids: Vec<&str> = box_ids
        .lines()
        .collect();
    ids.sort();
    for id in 0..(ids.len() - 1) {
        let (dist, same) = find_string_distance(ids[id], ids[id+1]);
        if dist == 1 {
            return same;
        }
    }
    ids.sort_by(|a, b| a.split_at(1).1.cmp(&(b.split_at(1).1)));
    String::from("Chokes.")
}
