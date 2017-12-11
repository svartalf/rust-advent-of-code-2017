extern crate aoc2017;

use aoc2017::days::day10;

fn main() {
    let input = aoc2017::read("input/day10.txt").unwrap();
//    let value = day10::part1::parse(&mut digits, &input);
//    println!("Day 10 part 1 value: {}", value);
    let value = day10::part2::parse(&input);
    println!("Day 10 part 2 value: {}", value);

    //    let (score, garbage) = day09::parse(&input);
//    println!("Day 09 part 1 score: {}", score);
//    println!("Day 09 part 2 garbage: {}", garbage);
}
