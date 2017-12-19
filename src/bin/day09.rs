extern crate aoc2017;

use aoc2017::days::day09;

fn main() {
    let input = aoc2017::read_trim("input/day09.txt").unwrap();

    let (score, garbage) = day09::parse(&input);
    println!("Day 09 part 1 score: {}", score);
    println!("Day 09 part 2 garbage: {}", garbage);
}
