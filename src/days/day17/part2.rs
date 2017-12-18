pub fn parse(input: &str, iterations: u32) -> u32 {
    let steps: usize = input.parse().unwrap();
    let mut idx = 0usize;
    let mut zero_idx = 0usize;
    let mut buffer_len: usize = 1;
    let mut expected_value = 0;
    for value in 1..iterations {
        for _ in 0..steps {
            idx += 1;
            if idx >= buffer_len {
                idx = 0;
            }
        }
        idx += 1;
        buffer_len += 1;

        if idx < zero_idx {
            zero_idx += 1;
        }
        if idx == zero_idx + 1 {
            expected_value = value;
        }
    }

    expected_value
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::parse;

    #[test]
    fn day17_part2_test1() {
        assert_eq!(638, parse("3", 2018));
    }
}
