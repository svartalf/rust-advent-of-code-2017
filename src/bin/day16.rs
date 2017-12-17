extern crate aoc2017;

use aoc2017::days::day16;

fn main() {
    let input = aoc2017::read("input/day16.txt").unwrap();
    let programs = "abcdefghijklmnop";
//    let value1 = day16::part1::parse(programs, &input, 1);
    let value2 = day16::part2::parse(programs, &input, 1_000_000_000);

//    println!("Day 16 part 1 value: {}", value1);
    println!("Day 16 part 2 value: {}", value2);
}
