use std::collections::{HashMap, HashSet};

use super::common::parse_input;

pub fn parse(input: &str) -> usize {
    let map = parse_input(input);

    let mut groups: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut already_checked: HashSet<usize> = HashSet::new();
    let mut working_on: Vec<(usize, usize)> = map.keys().map(|key| (*key, *key)).collect();

    loop {
        let (grouper, value) = match working_on.pop() {
            None => break,
            Some((grouper, value)) => {
                if already_checked.contains(&value) {
                    continue;
                } else {
                    already_checked.insert(value);
                }
                (grouper, value)
            },
        };

        let children = map.get(&value).unwrap();
        let entry = groups.entry(grouper).or_insert(HashSet::new());
        for child in children {
            (*entry).insert(*child);
            let value = (grouper, *child);
            if !working_on.contains(&value) {
                working_on.push(value);
            }
        }
    }

    groups.len()
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn day12_part2_test1() {
        let input = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";

        assert_eq!(2, parse(input));
    }

}

