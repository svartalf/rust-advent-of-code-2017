use days::day08::common::*;


pub fn search(data: &str) -> isize {
    let mut acc = Registry::new();
    let registry = data.lines()
        .map(Line::from)
        // TODO: Return Err() immediately
        .filter_map(|line| line.ok())
        .fold(&mut acc,  |acc, line| {
            acc.execute(line);
            acc
        });

    registry.highest()
}

#[cfg(test)]
mod tests {
    use super::search;

    #[test]
    fn day08_part1_test1() {
        let data = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10
";

        assert_eq!(10, search(data));
    }
}
