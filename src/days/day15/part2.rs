struct Generator {
    current: usize,
    factor: usize,
    multiply: usize,
}

impl Generator {
    pub fn new(current: usize, factor: usize, multiply: usize) -> Generator {
        Generator {
            current,
            factor,
            multiply,
        }
    }
}

impl Iterator for Generator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.current = (self.current * self.factor) % 2147483647;
            if self.current % self.multiply == 0 {
                return Some(self.current);
            }
        }
    }
}

pub fn parse(input: &str) -> usize {
    let start: Vec<_> = input.lines().map(|line | line.parse::<usize>().unwrap()).collect();
    let a = Generator::new(start[0], 16807, 4);
    let b = Generator::new(start[1], 48271, 8);

    a.zip(b)
        .take(5_000_000)
        .filter(|&(v1, v2)| {
            v1 & 0xffff == v2 & 0xffff
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::{Generator, parse};

    #[test]
    fn day15_part2_test1() {
        let mut generator = Generator::new(65, 16807, 4);

        assert_eq!(1352636452, generator.next().unwrap());
        assert_eq!(1992081072, generator.next().unwrap());
        assert_eq!(530830436, generator.next().unwrap());
        assert_eq!(1980017072, generator.next().unwrap());
        assert_eq!(740335192, generator.next().unwrap());
    }

    #[test]
    fn day15_part2_test2() {
        let mut generator = Generator::new(8921, 48271, 8);

        assert_eq!(1233683848, generator.next().unwrap());
        assert_eq!(862516352, generator.next().unwrap());
        assert_eq!(1159784568, generator.next().unwrap());
        assert_eq!(1616057672, generator.next().unwrap());
        assert_eq!(412269392, generator.next().unwrap());
    }

    #[test]
    fn day15_part2_test3() {
        assert_eq!(309, parse("65\n8921\n"));
    }

}
