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

impl Iterator for Diagram {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            None => {
                let start_y = self.source[0].iter()
                    .position(|cell| Cell::Vertical == *cell)
                    .unwrap();

                Some(self.source[0][start_y])
            },
            Some((x, y)) => {
                let (new_x, new_y) = match self.direction {
                    // TODO: Handle overflow and underflow
                    Direction::Up => (x, y - 1),
                    Direction::Right => (x + 1, y),
                    Direction::Down => (x, y - 1),
                    Direction::Left => (x - 1, y),
                };
                self.current = Some((new_x, new_y));
                let value = self.source[new_x][new_y];

                if let Cell::DirectionChange = value {
                    // TODO: Find new direction
                    // Only perpendicular directions should be checked
                }

                Some(value)
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
                            ' ' => Cell::Empty,
                            _ => unreachable!()
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
