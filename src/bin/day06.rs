extern crate aoc2017;

use aoc2017::days::day06;

fn main() {
    let input = aoc2017::read("input/day06.txt").unwrap();

    let steps = day06::part1::parse(&input);
    println!("Day 06 part 1 required steps: {}", steps);

    let steps = day06::part2::parse(&input);
    println!("Day 06 part 2 required steps: {}", steps);
}
