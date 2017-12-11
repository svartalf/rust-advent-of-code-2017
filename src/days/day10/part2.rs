pub fn parse(list: &mut Vec<usize>, input: &str) -> usize {
    let mut current = 0usize;
    let mut skip_size = 0usize;
    let mut lengths: Vec<usize> = input.bytes().map(|value| value as usize).collect();
    lengths.extend_from_slice(&[17, 31, 73, 47, 23]);

    for i in 0..64 {
        for length in &lengths[..] {
            let (indexes, mut values): (Vec<_>, Vec<_>) = list.iter()
                .enumerate().cycle()
                .skip(current).take(*length)
                .map(|(idx, item)| (idx, *item))
                .unzip();
            values.reverse();

            for (index, value) in indexes.iter().zip(values) {
                list[*index] = value;
            }

            current += length + skip_size;
            skip_size += 1;
        }
        println!("{:?}", list);
    }

    list.chunks(16)
        .map(|chunk| {
            let mut iterator = chunk.iter();
            let first = *iterator.next().unwrap();
            chunk.iter().fold(first, |acc, value| {
                acc ^ *value
            })
        });

    println!("{:?}", list);

    0
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn day10_part2_test1() {
        let mut list: Vec<usize> = vec![0, 1, 2, 3, 4];
        let input = "3,4,1,5";

        assert_eq!(12, parse(&mut list, input));
    }

}
