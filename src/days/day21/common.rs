use std::fmt::{self, Write};
use std::str::FromStr;

#[derive(Debug)]
struct Rule {
    width: usize,
    pattern: Vec<bool>,
    result: Vec<bool>,
}

impl Rule {

    pub fn is_match(&self, data: &[bool]) -> bool {
        false
    }

    fn parse(data: &str) -> Vec<bool> {
        data.split('/').flat_map(|row| {
            row.chars().map(|chr| {
                match chr {
                    '#' => true,
                    '.' => false,
                    _ => unreachable!(),
                }
            })
        }).collect()
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut r = s.split(" => ");
        let pattern = Rule::parse(r.next().unwrap());
        let width = (pattern.len() as f64).sqrt() as usize;
        let result = Rule::parse(r.next().unwrap());

        Ok(Rule {
            width,
            pattern,
            result,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::Rule;

    #[test]
    fn day21_common_parse_2x2() {
        let input = "../.. => .#./###/##.";

        let rule = Rule::from_str(input).unwrap();
        println!("{:?}", rule);

        assert_eq!(rule.width, 2);
        assert_eq!(vec![false, false, false, false], rule.pattern);
    }
}