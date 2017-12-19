extern crate aoc2017;

use aoc2017::days::day15;

fn main() {
    let input = aoc2017::read_trim("input/day15.txt").unwrap();

    let value1 = day15::part1::parse(&input);
    let value2 = day15::part2::parse(&input);

    println!("Day 15 part 1 value: {}", value1);
    println!("Day 15 part 2 value: {}", value2);
}
