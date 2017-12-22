use std::ops;
use std::default::Default;
use std::clone::Clone;
use std::fmt::{self, Write};

pub struct Grid<T>(Vec<Vec<T>>);

impl<T> Grid<T> where T: Default + Clone {
    pub fn new() -> Grid<T> {
        let size = u16::max_value() as usize;
        Grid(
            vec![vec![<T as Default>::default(); size]; size],
        )
    }

    // 0 - i16::max_value() / 2
    // 1 - i16::max_value() / 2 + 1
    // 1 - i16::max_value() / 2 - 1
    #[inline]
    fn cast_coords(&self, (x, y): (i16, i16)) -> (usize, usize) {
        let offset = (i16::max_value() / 2) as i32;
        (
            (x as i32 + offset) as usize,
            (y as i32 + offset) as usize,
        )
    }
}

impl<T> ops::Index<(i16, i16)> for Grid<T> where T: Default + Clone {
    type Output = T;

    #[inline]
    fn index(&self, coords: (i16, i16)) -> &Self::Output {
        let (x, y) = self.cast_coords(coords);

        &self.0[y][x]
    }
}

impl<T> ops::IndexMut<(i16, i16)> for Grid<T> where T: Default + Clone {

    #[inline]
    fn index_mut(&mut self, coords: (i16, i16)) -> &mut T {
        let (x, y) = self.cast_coords(coords);

        &mut self.0[y][x]
    }
}

impl fmt::Debug for Grid<bool> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut left = 0;
        let mut top = 0;
        let mut right = 0;
        let mut bottom = 0;

        for (x, row) in self.0.iter().enumerate() {
            for (y, item) in row.iter().enumerate() {
                if !item {
                    continue;
                }

                if x < left {
                    left = x;
                }
                if x > right {
                    right = x;
                }
                if y < top {
                    top = y;
                }
                if y > bottom {
                    bottom = y;
                }
            }
        }

        println!("{} {} {} {}", left, top, right, bottom);

        for y in top..bottom {
            for x in left..right {
                let chr = match self.0[x][y] {
                    true => '#',
                    false => '.',
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
    use super::Grid;

    #[test]
    fn day22_grid_create() {
        let mut grid: Grid<bool> = Grid::new();

        grid[(1, -1)] = true;
        grid[(-1, 0)] = true;
        // ..#
        // #..
        // ...

        println!("{:?}", grid);
    }
}