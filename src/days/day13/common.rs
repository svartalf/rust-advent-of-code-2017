#[derive(Debug, PartialEq)]
enum Direction {
    Forward,
    Backward,
}

pub struct Scanner<T> {
    data: Vec<T>,
    current: usize,
    direction: Direction,
}

impl<T> Scanner<T> {
    pub fn new(data: Vec<T>) -> Scanner<T> {
        Scanner {
            data: data,
            current: 0,
            direction: Direction::Forward,
        }
    }

    pub fn current(&self) -> &T {
        &self.data[self.current]
    }

    pub fn range(&self) -> usize {
        self.data.len()
    }

    pub fn next(&mut self) -> &T {
        match () {
            _ if self.current == 0 && self.direction == Direction::Backward => {
                self.direction = Direction::Forward;
                self.current = 1;
            },
            _ if self.current == self.data.len() - 1 && self.direction == Direction::Forward => {
                self.direction = Direction::Backward;
                self.current = self.data.len() - 2;
            },
            _ if self.direction == Direction::Backward => {
                self.current -= 1;
            },
            _ if self.direction == Direction::Forward => {
                self.current += 1;
            },
            _ => unreachable!()
        };

        &self.data[self.current]
    }
}
