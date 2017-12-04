extern crate aoc2017;

use aoc2017::days::day04;

fn main() {
    let input = aoc2017::read("input/day04.txt").unwrap();

    let valid = day04::part1::parse(&input);
    println!("Day 04 part 1 valid lines: {}", valid);

    let valid = day04::part2::parse(&input);
    println!("Day 04 part 2 valid lines: {}", valid);

}
