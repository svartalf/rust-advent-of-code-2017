use std::collections::{HashMap, VecDeque};

use days::day10::part2::parse as knot_hash;


#[derive(Debug, Clone)]
enum Cell {
    Unknown,
    Empty,
    Region(usize),
}


fn to_bits(byte: u32) -> Vec<bool> {
    let byte = byte.to_le();
    (0..4)
        .map(|n| byte & (1 << n) != 0)
        .rev() // TODO: Not fair
        .collect::<Vec<_>>()
}

pub fn parse(input: &str) -> usize {
    let mut grid: HashMap<(usize, usize), Cell> = HashMap::with_capacity(128 * 128);


    let iter = (0..128)
        .map(|line| knot_hash(&format!("{}-{}", input, line)))
        .map(|hash| {
            hash.chars()
                .map(|chr| chr.to_digit(16).unwrap())
                .flat_map(to_bits)
                .map(|bit| {
                    if bit {
                        Cell::Unknown
                    } else {
                        Cell::Empty
                    }
                })
                .enumerate()
                .collect::<Vec<_>>()
        })
        .enumerate();

    // TODO: Can I generate an `grid` from the iterator directly?
    for (y, row) in iter {
        for &(x, ref cell) in row.iter() {
            grid.insert((x, y), (*cell).clone());
        }
    }

    let mut regions = 0usize;

    // TODO: I got tired fighting with a borrow checker
    let keys = grid.keys().map(|&(x, y)| (x, y)).collect::<Vec<_>>();

    for &(x, y) in keys.iter() {
        match grid.get(&(x, y)).unwrap() {
            &Cell::Empty => continue,
            &Cell::Region(_) => continue,
            &Cell::Unknown => {
                println!("{}x{}", x, y);
                let mut current_region: Vec<(usize, usize)> = vec![];
                let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
                queue.push_back((x, y));

                loop {
                    if let Some((x, y)) = queue.pop_front() {
                        if x > 127 || y > 127 {
                            continue;
                        }

                        if current_region.contains(&(x, y)) {
                            continue;
                        }

                        match grid.get(&(x, y)) {
                            Some(&Cell::Unknown) => {
                                current_region.push((x, y));
                                println!("Got {}x{}", x, y);

                                if let Some(top_y) = y.checked_sub(1) {
                                    queue.push_back((x, top_y));
                                }
                                if let Some(right_x) = x.checked_add(1) {
                                    queue.push_back((right_x, y));
                                }
                                if let Some(bottom_y) = y.checked_add(1) {
                                    queue.push_back((x, bottom_y));
                                }
                                if let Some(left_x) = x.checked_sub(1) {
                                    queue.push_back((left_x, y));
                                }
                            },
                            _ => {}
                        }
                    } else {
                        println!("Queue empty");
                        break;
                    }
                }

                if current_region.len() > 0 {
                    println!("Got a new region #{} with {} elements", regions, current_region.len());
                    for key in current_region.iter() {
                        let entry = grid.entry(*key).or_insert(Cell::Unknown);
                        *entry = Cell::Region(regions);
                    }

                    regions += 1;
                }

            }
        }


    }

    println!("\n\n");
    for x in 0..128usize {
        for y in 0..128usize {
            match grid.get(&(x, y)) {
                Some(&Cell::Unknown) => print!("?"),
                Some(&Cell::Region(region)) => {
                    let repr = format!("{}", region);
                    print!("{}", repr.chars().last().unwrap());
                },
                _ => print!("."),
            }
        }
        println!("");
    }
    println!("\n\n");

    regions
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn day14_part2_test1() {
        let input = "flqrgnkx";

        assert_eq!(1242, parse(input));
    }

}
