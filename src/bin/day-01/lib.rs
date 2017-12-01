use traits::Circular;


pub fn parse(data: &str) -> usize {
    let mut chars = data.chars().circular().peekable();
    let mut sum: usize = 0;

    loop {
        let step: u32 = match (chars.next(), chars.peek()) {
            (Some(chr1), Some(chr2)) if chr1 == *chr2 => {
                chr1.to_digit(10).expect("Char to digit conversion failed")
            },
            (Some(_), Some(_)) => 0,
            _ => break
        };
        sum += step as usize;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn day_01_test_1() {
        assert_eq!(3, parse("1122"));
    }

    #[test]
    fn day_01_test_2() {
        assert_eq!(4, parse("1111"));
    }

    #[test]
    fn day_01_test_3() {
        assert_eq!(0, parse("1234"));
    }

    #[test]
    fn day_01_test_4() {
        assert_eq!(9, parse("91212129"));
    }
}
