use std::fmt::Debug;

#[derive(Debug)]
enum Direction {
    Start,
    Right,
    Top,
    Left,
    Bottom,
}

impl Direction {
    pub fn next(current: &Direction) -> Direction {
        match *current {
            Direction::Start => Direction::Right,
            Direction::Right => Direction::Top,
            Direction::Top => Direction::Left,
            Direction::Left => Direction::Bottom,
            Direction::Bottom => Direction::Right,
        }
    }
}

#[derive(Debug)]
struct Element<T> {
    value: T,
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Spiral<T> where T: Debug {
    data: Vec<Element<T>>,
    direction: Direction,
    req_steps: usize,
    left_steps: usize,
    turns: i8,
    x: isize,
    y: isize,
}

impl<T> Spiral<T> where T: Debug {
    pub fn new() -> Spiral<T> {
        Spiral {
            data: vec![],
            direction: Direction::Start,
            req_steps: 1,
            left_steps: 1,
            turns: 0,
            x: 0,
            y: 0,
        }
    }

    pub fn fill_with<F>(&mut self, mut filler: F)
            where F: FnMut(&Self) -> Option<T> {
        loop {
            match filler(&self) {
                None => break,
                Some(value) => {
                    let mut x = self.x;
                    let mut y = self.y;

                    if self.left_steps == 0 {
                        // new turn!
                        self.turns += 1;
                        self.direction = Direction::next(&self.direction);
                    }

                    if self.turns == 2 {
                        self.req_steps += 1;
                        self.left_steps = self.req_steps;
                        self.turns = 0;
                    }

                    match self.direction {
                        Direction::Start => {},
                        Direction::Right => x += 1,
                        Direction::Top => y += 1,
                        Direction::Left => x -= 1,
                        Direction::Bottom => y -= 1,
                    };

                    self.x = x;
                    self.y = y;
                    println!("Placing {:?} at ({}, {}) with {:?} and {}", value, x, y, self.direction, self.left_steps);

                    let element = Element {
                        value: value,
                        x: x,
                        y: y,
                    };
                    self.data.push(element);
                }
            }
        }
    }

}


#[cfg(test)]
mod tests {
    use super::Spiral;

    #[test]
    fn test_create() {
        let mut spiral: Spiral<u32> = Spiral::new();
    }

    #[test]
    fn test_fill() {
        let mut i = 0;
        let mut spiral: Spiral<u32> = Spiral::new();
        spiral.fill_with(|ref s| {
            i += 1;
            if i < 15 {
                Some(i)
            } else {
                None
            }
        });
        println!("{:#?}", spiral);
    }
}
