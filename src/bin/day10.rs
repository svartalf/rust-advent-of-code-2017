extern crate aoc2017;

use aoc2017::days::day10;

fn main() {
    let input = aoc2017::read_trim("input/day10.txt").unwrap();

    let value = day10::part1::parse(256, &input);
    println!("Day 10 part 1 value: {}", value);

    let value = day10::part2::parse(&input);
    println!("Day 10 part 2 value: {}", value);
}
