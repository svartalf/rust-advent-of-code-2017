use std::cmp;

pub fn parse(input: &str) -> (isize, isize) {
    let (x0, y0): (isize, isize) = (0, 0);
    let acc = ((x0, y0), 0isize);
    let ((x1, y1), max_distance): ((isize, isize), isize) = input.split(',')
        .map(|chr| {
            match chr {
                "n" => (0, 1),
                "ne" => (1, 1),
                "se" => (1, 0),
                "s" => (0, -1),
                "sw" => (-1, -1),
                "nw" => (-1, 0),
                _ => unreachable!(),
            }
        }).fold(acc, |((x, y), max): ((isize, isize), isize), (x1, y1): (isize, isize)| {
            (
                (x + x1, y + y1),
                cmp::max(max.abs(), cmp::max((x + x1).abs(), (y + y1).abs())),
            )
        });

    (
        cmp::max((x1 - x0).abs(), (y1 - y0).abs()),
        max_distance,
    )
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn day11_part1_test1() {
        let input = "ne,ne,ne";

        assert_eq!(3, parse(input).0);
    }

    #[test]
    fn day11_part1_test2() {
        let input = "ne,ne,sw,sw";

        assert_eq!(0, parse(input).0);
    }

    #[test]
    fn day11_part1_test3() {
        let input = "ne,ne,s,s";

        assert_eq!(2, parse(input).0);
    }

    #[test]
    fn day11_part1_test4() {
        let input = "se,sw,se,sw,sw";

        assert_eq!(3, parse(input).0);
    }

}
