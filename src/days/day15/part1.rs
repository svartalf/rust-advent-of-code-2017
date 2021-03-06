struct Generator {
    current: usize,
    factor: usize,
}

impl Generator {
    pub fn new(current: usize, factor: usize) -> Generator {
        Generator {
            current,
            factor,
        }
    }
}

impl Iterator for Generator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.current = (self.current * self.factor) % 2147483647;

        Some(self.current)
    }
}

pub fn parse(input: &str) -> usize {
    let start: Vec<_> = input.lines().map(|line | line.parse::<usize>().unwrap()).collect();
    let a = Generator::new(start[0], 16807);
    let b = Generator::new(start[1], 48271);

    a.zip(b)
        .take(40_000_000)
        .filter(|&(v1, v2)| {
            v1 & 0xffff == v2 & 0xffff
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::{Generator, parse};

    #[test]
    fn day15_part1_test1() {
        let mut generator = Generator::new(65, 16807);

        assert_eq!(1092455, generator.next().unwrap());
        assert_eq!(1181022009, generator.next().unwrap());
        assert_eq!(245556042, generator.next().unwrap());
        assert_eq!(1744312007, generator.next().unwrap());
        assert_eq!(1352636452, generator.next().unwrap());
    }

    #[test]
    fn day15_part1_test2() {
        let mut generator = Generator::new(8921, 48271);

        assert_eq!(430625591, generator.next().unwrap());
        assert_eq!(1233683848, generator.next().unwrap());
        assert_eq!(1431495498, generator.next().unwrap());
        assert_eq!(137874439, generator.next().unwrap());
        assert_eq!(285222916, generator.next().unwrap());
    }

    #[test]
    fn day15_part1_test3() {
        assert_eq!(588, parse("65\n8921\n"));
    }

}
