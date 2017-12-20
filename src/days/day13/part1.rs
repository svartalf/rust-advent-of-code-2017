use std::collections::HashMap;

use super::common::Scanner;

pub fn parse(input: &str) -> usize {
    let parsed: HashMap<_, _> = input.lines().map(|line| {
        let mut parts = line.splitn(2, ": ");
        let idx = parts.next().unwrap().parse::<usize>().unwrap();
        let length = parts.next().unwrap().parse::<usize>().unwrap();
        (idx, length)
    }).collect();

    let mut layers: HashMap<usize, _> = parsed.iter()
        .map(|(idx, length)| {
            let layer = (0..*length).collect::<Vec<_>>();
            (*idx, Scanner::new(layer))
        })
        .collect();

    let mut position: isize = -1;
    let max_pos = *parsed.keys().max().unwrap();
    let mut matches: Vec<usize> = vec![];

    loop {
        position += 1;
        for (layer, scanner) in layers.iter_mut() {
            if *layer == position as usize && *scanner.current() == 0 {
                matches.push(*layer * scanner.range());
            }

            (*scanner).next();
        }

        if position as usize >= max_pos {
            break;
        }
    }

    matches.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn day13_part1_test1() {
        let input = "0: 3
1: 2
4: 4
6: 4";

        assert_eq!(24, parse(input));
    }
}
