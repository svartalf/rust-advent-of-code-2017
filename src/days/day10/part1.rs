pub fn parse(input: &str) -> usize {
    let mut list: Vec<usize> = (0..256).collect();
    let mut current = 0usize;
    let mut skip_size = 0usize;
    let lengths: Vec<usize> = input.split(",").map(|part| {
        part.parse::<usize>().unwrap()
    }).collect();

    for length in lengths {
        let (indexes, mut values): (Vec<_>, Vec<_>) = list.iter()
            .enumerate().cycle()
            .skip(current).take(length)
            .map(|(idx, item)| (idx, *item))
            .unzip();
        values.reverse();

        for (index, value) in indexes.iter().zip(values) {
            list[*index] = value;
        }

        current += length + skip_size;
        skip_size += 1;
    }

    list.iter().take(2).fold(1, |acc, value| acc * value)
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn day10_part1_test1() {
        let mut list: Vec<usize> = vec![0, 1, 2, 3, 4];
        let input = "3,4,1,5";

        assert_eq!(12, parse(input));
    }

}
