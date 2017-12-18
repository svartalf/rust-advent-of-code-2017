extern crate aoc2017;

use aoc2017::days::day18;

fn main() {
    let input = aoc2017::read("input/day18.txt").unwrap();
    let value1 = day18::part1::parse(&input);

    println!("Day 18 part 1 value: {}", value1);
}
