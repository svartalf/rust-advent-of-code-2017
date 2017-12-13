use std::collections::HashMap;

enum Direction {
    Forward,
    Backward,
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct ScannerState<I> where I: Iterator {
    iter: I,
    orig: I,
    current: Option<<I as Iterator>::Item>,
    direction: Direction,
}

impl<I> ScannerState<I> where I: Clone + DoubleEndedIterator {
    pub fn current(&self) -> &Option<<I as Iterator>::Item> {
        &self.current
    }
}

impl<I> Iterator for ScannerState<I> where I: Clone + DoubleEndedIterator {
    type Item = <I as Iterator>::Item;

    #[inline]
    fn next(&mut self) -> Option<<I as Iterator>::Item> {
        let try_next = match self.direction {
            Direction::Forward => self.iter.next(),
            Direction::Backward => self.iter.next_back(),
        };

        match try_next {
            None => {
                self.direction = match self.direction {
                    Direction::Forward => Direction::Backward,
                    Direction::Backward => Direction::Forward,
                };

                self.iter = self.orig.clone();
                self.next();
                self.next()
            }
            value => value,
        }
    }
}

pub trait Scanner: Iterator {
    #[inline]
    fn scanner(self) -> ScannerState<Self>
        where Self: Sized + Clone {
            ScannerState {
                iter: self.clone(),
                orig: self,
                current: None,
                direction: Direction::Forward,
            }
    }
}

impl<T: Iterator> Scanner for T {}


pub fn parse(input: &str) -> usize {
    let parsed: HashMap<_, _> = input.lines().map(|line| {
        let mut parts = line.splitn(2, ": ");
        let idx = parts.next().unwrap().parse::<usize>().unwrap();
        let length = parts.next().unwrap().parse::<usize>().unwrap();
        (idx, length)
    }).collect();

    let mut layers: HashMap<usize, _> = parsed.iter()
        .map(|(idx, length)| {
            (*idx, (0..*length).scanner())
        })
        .collect();

    let mut position = 1usize;
    let max_pos = *parsed.keys().max().unwrap();
    let mut matches: Vec<usize> = vec![];

    loop {
        position += 1;

        for (layer, scanner) in layers.iter_mut() {
            if scanner.current().unwrap() == position {
                let depth = parsed.get(layer).unwrap_or(&0);
                matches.push(*layer * depth);
            }
            (*scanner).next();
        }

        if position >= max_pos {
            break;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn day13_part1_test1() {
        let input = "0: 3
1: 2
4: 4
6: 4";

        assert_eq!(6, parse(input));
    }
}
