#[allow(unused_imports)]

use std::collections::BTreeSet;
use std::fs;

pub fn day_1() -> () {
    let input = fs::read_to_string("lib/inputs/1.txt")
        .expect("Not a file");
    let res = add_freqs(&input);
    println!("{}", res);
    let res2 = find_first_repeat(&input);
    println!("Part 1: {}.  Part 2: {}", res, res2);
}

fn add_freqs(freq_string: &str) -> i32 {
    let mut freq = 0;
    for line in freq_string.lines() { 
        let res = line.parse::<i32>()
            .expect("Whoops");
        freq += res;
    };
    freq
}

fn find_first_repeat(freq_string: &str) -> i32 {
    let mut freq = 0;
    let lines: Vec<i32> = freq_string.lines().map(|line| line.parse::<i32>().expect("Whoops")).collect();
    let len_lines = lines.len();
    let mut sums: BTreeSet<i32> = BTreeSet::new();
    sums.insert(freq);
    let mut pos = 0;
    loop {
        freq += lines.get(pos).expect("Whoops");
        let is_present = sums.insert(freq);
        match is_present {
            false => return freq,
            true => pos = (pos + 1) % len_lines,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_frequency() {
        let freq_changes = "\
+5
-4
+3
";
    assert_eq!(add_freqs(freq_changes), 4);
    }

    #[test]
    fn test_find_repeat() {
        let freq_changes = "\
+5
-4
+3
";
    assert_eq!(find_first_repeat(freq_changes), 5);      
    }
}
