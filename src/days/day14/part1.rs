use days::day10::part2::parse as knot_hash;


fn to_bits(byte: u32) -> Vec<bool> {
    let byte = byte.to_le();
    (0..4)
        .map(|n| byte & (1 << n) != 0)
        .rev() // TODO: Not fair
        .collect::<Vec<_>>()
}

pub fn parse(input: &str) -> usize {
    (0..128)
        .map(|line| format!("{}-{}", input, line))
        .map(|line| knot_hash(&line))
        .flat_map(|hash| {
            hash.chars()
                .map(|chr| chr.to_digit(16).unwrap())
                .flat_map(to_bits)
                .collect::<Vec<_>>()
        })
        .filter(|bit| *bit == true)
        .count()
}

#[cfg(test)]
mod tests {
    use super::parse;
    use super::to_bits;

    #[test]
    fn day14_part1_test1() {
        let input = "flqrgnkx";

        assert_eq!(8108, parse(input));
    }

    #[test]
    fn day14_part1_test2() {
        let bits = to_bits('0'.to_digit(16).unwrap());
        assert_eq!(vec![false, false, false, false], bits);
    }
    #[test]
    fn day14_part1_test3() {
        let bits = to_bits('1'.to_digit(16).unwrap());
        assert_eq!(vec![false, false, false, true], bits);
    }
    #[test]
    fn day14_part1_test4() {
        let bits = to_bits('e'.to_digit(16).unwrap());
        assert_eq!(vec![true, true, true, false], bits);
    }
    #[test]
    fn day14_part1_test5() {
        let bits = to_bits('f'.to_digit(16).unwrap());
        assert_eq!(vec![true, true, true, true], bits);
    }
}
