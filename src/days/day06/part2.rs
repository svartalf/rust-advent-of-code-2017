use std::collections::HashMap;

fn get_max(memory: &[usize]) -> (usize, usize) {
    let mut idx = 0usize;
    let mut value = 0usize;

    for (idx0, value0) in memory.iter().enumerate() {
        if value0 > &value {
            value = *value0;
            idx = idx0;
        }
    }

    (idx, value)
}

pub fn parse(data: &str) -> usize {
    let mut memory: Vec<usize> = data.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
    let mut cycles: HashMap<Vec<usize>, usize> = HashMap::new();

    let mut steps = 0usize;

    loop {
        let (idx, mut left) = get_max(&memory);

        memory[idx] = 0;

        let mut next_idx = idx + 1;
        loop {
            if left == 0 {
                break;
            }

            if next_idx >= memory.len() {
                next_idx = 0;
            }

            memory[next_idx] += 1;
            left -= 1;
            next_idx += 1;
        }

        if cycles.contains_key(&memory) {
            let prev_steps = cycles.get(&memory).unwrap();
            return steps - prev_steps;
        } else {
            cycles.insert(memory.clone(), steps);
        }

        steps += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn day06_part1_test1() {
        let data = "0 2 7 0";

        assert_eq!(4, parse(data));
    }
}
