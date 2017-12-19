extern crate aoc2017;

use aoc2017::days::day19;

fn main() {
    let input = aoc2017::read("input/day19.txt").unwrap();
    let value1 = day19::part1::parse(&input);
    let value2 = day19::part2::parse(&input);

    println!("Day 19 part 1 value: {}", value1);
    println!("Day 19 part 2 value: {}", value2);
}
