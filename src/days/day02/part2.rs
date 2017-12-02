pub fn parse(data: &str) -> usize {
    data.lines().map(|line| {
        let row: Vec<usize> = line.split_whitespace().map(|chr| {
            chr.parse::<usize>().unwrap()
        }).collect();

        for i in 0..row.len() {
            for j in 0..row.len() {
                if i == j {
                    continue
                }
                if row[i] % row[j] == 0 {
                    return row[i] / row[j];
                }
                if row[j] % row[i] == 0 {
                    return row[j] / row[i];
                }
            }
        }

        0
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn day02_part2_test1() {
        let input: &str = "5 9 2 8\n9 4 7 3\n3 8 6 5";
        assert_eq!(9, parse(input));
    }

}
