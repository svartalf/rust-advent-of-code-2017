use std::iter::Iterator;
use std::collections::HashMap;


fn sum(spiral: &HashMap<(isize, isize), usize>, x: isize, y: isize) -> usize {
    let mut sum = 0usize;
    for i in x-1..x+2 {
        for j in y-1..y+2 {
            sum += match spiral.get(&(i, j)) {
                Some(value) => *value,
                None => 0,
            };
        }
    }

    sum
}

pub fn search(needle: usize) -> usize {
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut spiral: HashMap<(isize, isize), usize> = HashMap::new();
    spiral.insert((0, 0), 1);

    let res = (1usize..).filter(|x| x % 2 != 0).flat_map(|elem| {
        let mut round: Vec<usize> = vec![];
        for _ in 0..elem {
            x += 1;
            let value = sum(&spiral, x, y);
            spiral.insert((x, y), value);
            round.push(value);
        }
        for _ in 0..elem {
            y -= 1;
            let value = sum(&spiral, x, y);
            spiral.insert((x, y), value);
            round.push(value);
        }
        for _ in 0..elem+1 {
            x -= 1;
            let value = sum(&spiral, x, y);
            spiral.insert((x, y), value);
            round.push(value);
        }
        for _ in 0..elem+1 {
            y += 1;
            let value = sum(&spiral, x, y);
            spiral.insert((x, y), value);
            round.push(value);
        }

        round
    });

    for x in res {
        if x > needle {
            return x;
        }
    }
    0
}
