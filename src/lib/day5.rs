use std::cmp;

fn opposing_polarity(u1: char, u2: char) -> bool {
    let lower_one = u1.to_lowercase().next().unwrap();
    let lower_two = u2.to_lowercase().next().unwrap();
    lower_one == lower_two && u1 != u2
}

pub fn polymer_reaction(input: &str, maybe_ignore: Option<char>) -> usize {
    let mut ignored = vec!['\n'];
    if let Some(x) = maybe_ignore {
        ignored.push(x.to_lowercase().next().unwrap());
        ignored.push(x.to_uppercase().next().unwrap());
    }
    let ignored = ignored;
    let mut input_chars = input.chars();
    let mut dest: Vec<char> = Vec::new();
    loop {
        let first: char;
        let second: char;
        match input_chars.next() {
            None => return dest.len(),
            Some(c) => if ignored.contains(&c) {
                continue;
            } else {
                second = c;
            }
        }
        if let Some(x) = dest.pop() {
            first = x;
        } else {
	    dest.push(second);
            continue;
        }
        if !opposing_polarity(first, second) {
            dest.push(first);
            dest.push(second);
        }
    }
}

pub fn optimal_removal(input: &str) -> usize {
    let mut lowercases: Vec<char> = input.to_ascii_lowercase()
        .chars()
        .collect();
    lowercases.sort_unstable();
    lowercases.dedup();
    let mut lowest_len = input.len();
    for c in lowercases.iter() {
        lowest_len = cmp::min(lowest_len, polymer_reaction(input, Some(*c)));
    }
    lowest_len 
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_polarity() {
        assert_eq!(opposing_polarity('a', 'A'), true);
        assert_eq!(opposing_polarity('a', 'B'), false);
        assert_eq!(opposing_polarity('a', 'a'), false);
    }

    #[test]
    fn test_reaction_no_ignore() {
        let input1 = String::from("abBA\n");
        assert_eq!(polymer_reaction(&input1, None), 0);
        let input2 = String::from("dabAcCaCBAcCcaDA");
        assert_eq!(polymer_reaction(&input2, None), 10);
    }

    #[test]
    fn test_reaction_ignore() {
        let input1 = String::from("abcBA");
        assert_eq!(polymer_reaction(&input1, Some('a')), 3);
        assert_eq!(polymer_reaction(&input1, Some('d')), 5);
        assert_eq!(polymer_reaction(&input1, Some('c')), 0);
        assert_eq!(polymer_reaction(&input1, Some('C')), 0);
    }

    #[test]
    fn test_optimal_removal() {
        let input1 = String::from("abcBA");
        assert_eq!(optimal_removal(&input1), 0);
        let input2 = String::from("dabAcCaCBAcCcaDA");
        assert_eq!(optimal_removal(&input2), 4);
    }
}
