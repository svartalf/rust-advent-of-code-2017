pub fn parse(input: &str) -> String {
    let mut list = (0..256).collect::<Vec<usize>>();

    let mut current = 0usize;
    let mut skip_size = 0usize;
    let mut lengths: Vec<usize> = input.bytes().map(|value| value as usize).collect();
    lengths.extend_from_slice(&[17, 31, 73, 47, 23]);

    for _ in 0..64 {
        for length in &lengths[..] {
            let local_current = current % list.len();
            let (indexes, mut values): (Vec<_>, Vec<_>) = list.iter()
                .enumerate().cycle()
                .skip(local_current).take(*length)
                .map(|(idx, item)| (idx, *item))
                .unzip();
            values.reverse();

            for (index, value) in indexes.iter().zip(values) {
                list[*index] = value;
            }

            current += length + skip_size;
            skip_size += 1;
        }
    }

    let res: Vec<String> = list.chunks(16)
        .map(|chunk| {
            let mut iterator = chunk.iter();
            let first = *iterator.next().unwrap();
            iterator.fold(first, |acc, value| {
                acc ^ *value
            })
        }).map(|byte| {
            format!("{:02x}", byte)
        }).collect();

    res.join("")
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn day10_part2_test1() {
        let input = "";
        assert_eq!("a2582a3a0e66e6e86e3812dcb672a272".to_string(), parse(input));
    }

    #[test]
    fn day10_part2_test2() {
        let input = "AoC 2017";
        assert_eq!("33efeb34ea91902bb2f59c9920caa6cd".to_string(), parse(input));
    }

    #[test]
    fn day10_part2_test3() {
        let input = "1,2,3";
        assert_eq!("3efbe78a8d82f29979031a4aa0b16a9d".to_string(), parse(input));
    }

    #[test]
    fn day10_part2_test4() {
        let input = "1,2,4";
        assert_eq!("63960835bcdc130f0b66d7ff4f6a5a8e".to_string(), parse(input));
    }

}
