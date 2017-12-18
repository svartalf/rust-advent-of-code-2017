use std::time::Instant;

pub fn parse(input: &str, iterations: u32, search_for: u32) -> u32 {
    let now = Instant::now();
    let steps: usize = input.parse().unwrap();
    let mut buffer: Vec<u32> = Vec::with_capacity(iterations as usize);
    buffer.push(0);
    let mut idx = 0usize;
    for value in 1..iterations {
        for _ in 0..steps {
            idx += 1;
            if idx >= buffer.len() {
                idx = 0;
            }
        }
        idx += 1;
        buffer.insert(idx, value);

        if value % 10_000 == 0 {
            println!("Iter {} at {}s", value, now.elapsed().as_secs());
        }
    }

    let idx = buffer.iter()
        .position(|val| *val == search_for)
        .unwrap();

    buffer[idx + 1]
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::parse;

    #[test]
    fn day17_part1_test1() {
        assert_eq!(638, parse("3", 2018, 2017));
    }
}
