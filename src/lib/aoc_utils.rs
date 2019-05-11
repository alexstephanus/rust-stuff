use std::fmt;
use std::fs;

pub fn get_day_input(day: i32) -> String {
    let filename = format!("lib/inputs/{}.txt", day);
    let error_msg = format!("Error opening file {}", filename);
    let res = fs::read_to_string(filename).expect(&error_msg);
    println!("{}", res.len());
    res
}
