pub fn parse(data: &str) -> usize {
    let mut offsets = data.lines().map(|line| {
        line.trim().parse::<isize>().unwrap()
    }).collect::<Vec<isize>>();

    let length = offsets.len();
    let mut idx: usize = 0;
    let mut steps: usize = 0;
    loop {
        let offset = offsets[idx];
        offsets[idx] += 1;
        idx = (idx as isize + offset) as usize;
        steps += 1;

        if idx as usize >= length {
            return steps;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn day05_part1_test1() {
        let data = "0\n3\n0\n1\n-3";

        assert_eq!(5, parse(data));
    }
}
