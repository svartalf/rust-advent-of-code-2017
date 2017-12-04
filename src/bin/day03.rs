extern crate aoc2017;

use aoc2017::days::day03;

fn main() {
    let input = aoc2017::read("input/day03.txt").unwrap().parse::<usize>().unwrap();

    let distance = day03::part1::distance(input);
    println!("Day 02 part 1 steps: {}", distance);

    let distance = day03::part2::search(input);
    println!("Day 02 part 2 next one element: {}", distance);
}
