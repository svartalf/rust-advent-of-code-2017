pub fn parse(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn day13_part2_test1() {
        let input = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";

        assert_eq!(2, parse(input));
    }

}

