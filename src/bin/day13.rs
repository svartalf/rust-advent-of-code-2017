extern crate aoc2017;

use aoc2017::days::day13;

fn main() {
    let input = aoc2017::read("input/day13.txt").unwrap();

    let value1 = day13::part1::parse(&input);
    let value2 = day13::part2::parse(&input);

    println!("Day 13 part 1 value: {}", value1);
    println!("Day 13 part 2 value: {}", value2);
}
