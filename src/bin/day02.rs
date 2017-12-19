extern crate aoc2017;

use aoc2017::days::day02;

fn main() {
    let input = aoc2017::read_trim("input/day02.txt").unwrap();

    let checksum = day02::part1::parse(&input);
    println!("Day 02 part 1 spreadsheet checksum: {}", checksum);

    let checksum = day02::part2::parse(&input);
    println!("Day 02 part 2 spreadsheet checksum: {}", checksum);

}
