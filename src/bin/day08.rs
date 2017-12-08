extern crate aoc2017;

use aoc2017::days::day08;

fn main() {
    let input = aoc2017::read("input/day08.txt").unwrap();

    let maxed = day08::part1::search(&input);
    println!("Day 08 part 1 max value: {}", maxed);

//    let steps = day08::part2::parse(&input);
//    println!("Day 08 part 2 required steps: {}", steps);
}
