extern crate aoc2017;

use aoc2017::days::day20;

fn main() {
    let input = aoc2017::read("input/day20.txt").unwrap();
    let value1 = day20::part1::parse(&input, 10_000);
    let value2 = day20::part2::parse(&input, 10_000);

    println!("Day 20 part 1 value: {}", value1);
    println!("Day 20 part 2 value: {}", value2);
}
