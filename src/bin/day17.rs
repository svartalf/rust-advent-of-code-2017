extern crate aoc2017;

use aoc2017::days::day17;

fn main() {
    let input = aoc2017::read_trim("input/day17.txt").unwrap();
    let value1 = day17::part1::parse(&input, 2018, 2017);
    let value2 = day17::part2::parse(&input, 50_000_000);

    println!("Day 17 part 1 value: {}", value1);
    println!("Day 17 part 2 value: {}", value2);
}
