extern crate aoc2017;

use aoc2017::days::day08;

fn main() {
    let input = aoc2017::read_trim("input/day08.txt").unwrap();

    let max = day08::part1::search(&input);
    println!("Day 08 part 1 max value: {}", max);

    let max = day08::part2::search(&input);
    println!("Day 08 part 2 max value: {}", max);
}
