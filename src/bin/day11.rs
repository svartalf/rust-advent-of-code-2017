extern crate aoc2017;

use aoc2017::days::day11;

fn main() {
    let input = aoc2017::read_trim("input/day11.txt").unwrap();

    let (value, distance) = day11::parse(&input);
    println!("Day 11 part 1 value: {}", value);
    println!("Day 11 part 2 distance: {}", distance);
}
