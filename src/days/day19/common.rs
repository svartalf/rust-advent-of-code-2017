use std::fmt::{self, Write};
use std::str::FromStr;
use std::iter::Iterator;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Cell {
    Vertical,           // |
    Horizontal,         // -
    DirectionChange,    // +
    Letter(char),       // ex. A
    Empty,
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub struct Diagram {
    source: Vec<Vec<Cell>>,
    direction: Direction,
    current: Option<(usize, usize)>
}

impl Diagram {

    #[inline]
    fn probe(&self, x: usize, y: usize) -> Cell {
        let row = self.source.get(y);
        if row.is_none() {
            return Cell::Empty;
        }
        let cell = row.unwrap().iter().nth(x);
        if cell.is_none() {
            return Cell::Empty;
        }

        *cell.unwrap()
    }

    // Based on the `self.current` (should be '+')
    fn get_direction(&self) -> Direction {
        let (x, y) = self.current.unwrap();
        match self.direction {
            Direction::Up | Direction::Down => {
                if Cell::Empty != self.probe(x - 1, y) {
                    return Direction::Left;
                }
                if Cell::Empty != self.probe(x + 1, y) {
                    return Direction::Right;
                }
            },
            Direction::Left | Direction::Right => {
                if Cell::Empty != self.probe(x, y - 1) {
                    return Direction::Up;
                }
                if Cell::Empty != self.probe(x, y + 1) {
                    return Direction::Down;
                }
            }
        }

        unreachable!()
    }
}

impl Iterator for Diagram {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            None => {
                let y = 0;
                let x = self.source[y].iter()
                    .position(|cell| *cell != Cell::Empty)
                    .unwrap();

                assert_ne!(Cell::Empty, self.source[y][x]);

                self.current = Some((x, y));
                Some(self.source[y][x])
            },
            Some((x, y)) => {
                if let Cell::DirectionChange = self.source[y][x] {
                    self.direction = self.get_direction();
                }

                let (new_x, new_y) = match self.direction {
                    Direction::Up => (x, y - 1),
                    Direction::Right => (x + 1, y),
                    Direction::Down => (x, y + 1),
                    Direction::Left => (x - 1, y),
                };

                match self.source[new_y][new_x] {
                    Cell::Empty => {
                        None
                    },
                    value => {
                        self.current = Some((new_x, new_y));
                        Some(value)
                    }
                }
            }
        }
    }
}

impl FromStr for Diagram {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let source = s.lines()
            .map(|line| {
                line.chars()
                    .map(|chr| {
                        match chr {
                            '|' => Cell::Vertical,
                            '-' => Cell::Horizontal,
                            '+' => Cell::DirectionChange,
                            letter @ 'A'...'Z' => Cell::Letter(letter),
                            _ => Cell::Empty,
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Ok(Diagram {
            source,
            direction: Direction::Down,
            current: None,
        })
    }
}

impl fmt::Debug for Diagram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.source {
            for cell in row {
                let chr = match *cell {
                    Cell::Vertical => '|',
                    Cell::Horizontal => '-',
                    Cell::DirectionChange => '+',
                    Cell::Letter(letter) => letter,
                    _ => ' '
                };
                f.write_char(chr)?;
            }
            f.write_char('\n')?;
        }
        f.write_char('\n')
    }
}


#[cfg(test)]
mod tests {
}
