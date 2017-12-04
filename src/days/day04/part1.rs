use std::collections::HashSet;

fn is_valid(line: &str) -> bool {
    let mut acc: HashSet<&str> = HashSet::new();
    for part in line.split_whitespace() {
        if acc.contains(part) {
            return false;
        } else {
            acc.insert(part);
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
        let data = "aa bb cc dd ee\naa bb cc dd aa\naa bb cc dd aaa";

        assert_eq!(2, parse(data));
    }
}
