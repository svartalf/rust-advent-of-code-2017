use std::collections::{HashMap, HashSet};

use super::common::parse_input;

pub fn parse(input: &str) -> usize {
    let map = parse_input(input);

    let mut already_checked: HashSet<usize> = HashSet::new();
    let mut working_on = vec![0];

    loop {
        let el = match working_on.pop() {
            None => break,
            Some(value) => {
                already_checked.insert(value);
                value
            },
        };

        for child in map.get(&el).unwrap() {
            if !already_checked.contains(child) {
                working_on.push(*child);
            }
        }
    }

    already_checked.len()
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn day12_part1_test1() {
        let input = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";

        assert_eq!(6, parse(input));
    }
}
