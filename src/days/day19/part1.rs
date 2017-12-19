use std::str::FromStr;

use super::common::{Diagram, Cell};


pub fn parse(data: &str) -> String {
    let diagram = Diagram::from_str(data).unwrap();
    println!("{:?}", diagram);
    diagram
        .inspect(|cell| println!("{:?}", cell))
        .filter_map(|cell| {
            match cell {
                Cell::Letter(letter) => Some(letter),
                _ => None,
            }
        })
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn day19_part1_test1() {
        let input = include_str!("test_input.txt");
        assert_eq!("ABCDEF".to_string(), parse(input));
    }
}
