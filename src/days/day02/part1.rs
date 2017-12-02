pub fn parse(data: &str) -> usize {
    data.lines().map(|line| {
        let row: Vec<usize> = line.split_whitespace().map(|chr| {
            chr.parse::<usize>().unwrap()
        }).collect();
        let max = row.iter().max().unwrap();
        let min = row.iter().min().unwrap();
        max - min
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn day02_part1_test1() {
        let spreadsheet = "5 1 9 5\n7 5 3\n2 4 6 8";

        assert_eq!(18, parse(spreadsheet));
    }
}
