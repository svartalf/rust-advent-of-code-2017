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

    let max_pos = *parsed.keys().max().unwrap();

    // Stupid bruteforce, highly unefficient
    'iteration: loop {
        let mut iter_position = position;
        println!("Iteration #{}", position);
        // TODO: Get rid of the wait period simulation at all
        for scanner in layers.values_mut() {
            (*scanner).reset();
        }

        'probe: loop {
            iter_position += 1;
            for (layer, scanner) in layers.iter_mut() {
                if *layer as isize == iter_position && *scanner.current() == 0 {
                    position -= 1;
                    break 'probe;
                }

                (*scanner).next();
            }

            if iter_position >= max_pos as isize {
                break 'iteration;
            }
        }
    }

    // `-1` since we started from -1 offset and should compensate it
    (position.abs() as usize) - 1
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
