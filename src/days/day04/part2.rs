use std::collections::HashSet;

fn is_valid(line: &str) -> bool {
    let mut acc: HashSet<Vec<char>> = HashSet::new();
    for part in line.split_whitespace() {
        let mut chars = part.chars().collect::<Vec<char>>();
        chars.sort();

        if acc.contains(&chars) {
            return false;
        } else {
            acc.insert(chars);
        }
    }
    return true;
}

pub fn parse(data: &str) -> usize {
    data.lines().map(|line| {
        if is_valid(line) {
            return 1;
        } else {
            return 0;
        }
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn day04_part1_test1() {
        let data = "abcde fghij\nabcde xyz ecdab\na ab abc abd abf abj\niiii oiii ooii oooi oooo\noiii ioii iioi iiio";

        assert_eq!(3, parse(data));
    }
}
