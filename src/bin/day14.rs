extern crate aoc2017;

use aoc2017::days::day14;

fn main() {
    let input = aoc2017::read_trim("input/day14.txt").unwrap();

    let value1 = day14::part1::parse(&input);
    let value2 = day14::part2::parse(&input);

    println!("Day 14 part 1 value: {}", value1);
    println!("Day 12 part 2 value: {}", value2);
}
