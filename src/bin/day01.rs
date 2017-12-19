extern crate aoc2017;

use aoc2017::days::day01::{part1, part2};

fn main() {
    let input = aoc2017::read_trim("input/day01.txt").unwrap();

    let result = part1::parse(&input);
    println!("Day 01 part 1 CAPTCHA result: {}", result);

    let result = part2::parse(&input);
    println!("Day 01 part 2 CAPTCHA result: {}", result);
}
