use std::str::FromStr;

use super::common::{Diagram, Cell};


pub fn parse(data: &str) -> String {
    let diagram = Diagram::from_str(data).unwrap();
    for cell in diagram {
        println!("{:?}", cell);
    }
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn day18_part1_test1() {
        let input = "     |
     |  +--+
     A  |  C
 F---|----E|--+
     |  |  |  D
     +B-+  +--+
";
        assert_eq!("ABCDE".to_string(), parse(input));
    }
}
