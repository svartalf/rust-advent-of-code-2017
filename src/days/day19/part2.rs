use std::str::FromStr;

use super::common::Diagram;


pub fn parse(data: &str) -> usize {
    let diagram = Diagram::from_str(data).unwrap();

    diagram.count()
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn day19_part2_test1() {
        let input = include_str!("test_input.txt");
        assert_eq!(38, parse(input));
    }
}
