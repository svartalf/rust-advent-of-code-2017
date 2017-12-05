extern crate aoc2017;

use aoc2017::days::day05;

fn main() {
    let input = aoc2017::read("input/day05.txt").unwrap();

    let steps = day05::part1::parse(&input);
    println!("Day 05 part 1 required steps: {}", steps);

    let steps = day05::part2::parse(&input);
    println!("Day 05 part 2 required steps: {}", steps);
}
