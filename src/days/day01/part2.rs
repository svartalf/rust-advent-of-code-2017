pub fn parse(data: &str) -> usize {
    let chars = data.chars().cycle();
    let steps = data.len() / 2;

    data.chars().enumerate().map(|(idx, chr)| {
        let picked = chars.clone().nth(idx + steps)
            .expect("Infinite iterator yield infinitely");
        if chr == picked {
            chr.to_digit(10).expect("Char to digit conversion failed") as usize
        } else {
            0
        }
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn day01_part2_test1() {
        assert_eq!(6, parse("1212"));
    }

    #[test]
    fn day01_part2_test2() {
        assert_eq!(0, parse("1221"));
    }

    #[test]
    fn day01_part2_test3() {
        assert_eq!(4, parse("123425"));
    }

    #[test]
    fn day01_part2_test4() {
        assert_eq!(12, parse("123123"));
    }

    #[test]
    fn day01_part2_test5() {
        assert_eq!(4, parse("12131415"));
    }
}
