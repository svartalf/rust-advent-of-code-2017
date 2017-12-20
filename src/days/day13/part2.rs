use std::collections::HashMap;

pub fn parse(input: &str) -> usize {
    let parsed: HashMap<_, _> = input.lines().map(|line| {
        let mut parts = line.splitn(2, ": ");
        let idx = parts.next().unwrap().parse::<usize>().unwrap();
        let length = parts.next().unwrap().parse::<usize>().unwrap();
        (idx, length)
    }).collect();

    (0usize..).filter(|wait| {
        for (layer, depth) in parsed.iter() {
            let position = layer + wait;
            let scanner_position = 2 * depth - 2;
            if position % scanner_position == 0 {
                return false;
            }
        }
        true
    }).next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn day13_part2_test1() {
        let input = "0: 3
1: 2
4: 4
6: 4";

        assert_eq!(10, parse(input));
    }
}
