# Advent of code 2017

Me trying to solve [AoC 2017](http://adventofcode.com/2017) with [Rust](https://www.rust-lang.org).
Most important goal is to create idiomatic Rust code here (if I'll have enough time for that).

Also learn some stuff.

## Running (nightly only)

1. Run tests suite for all days

```bash
cargo test
```

2. Run specific day solver

```bash
cargo run --bin day01
```

## Results

| Day                                        | Notes                                                                |
| ------------------------------------------ | -------------------------------------------------------------------- |
| 1. Inverse Captcha                         | Learn iterators the hard way                                         |
| 2. Corruption Checksum                     | Iterators again                                                      |
| 3. Spiral Memory                           | Terrible solution, should be reworked                                |
| 4. High-Entropy Passphrases                | Iterators once again                                                 |
| 5. A Maze of Twisty Trampolines, All Alike | Easy and plain one                                                   |
| 6. Memory Reallocation                     | Easy, but custom iterator should be implemented instead of `get_max` |
| 7. Recursive Circus                        | Still working on a proper implementation with arena-based hashmap    |
| 8. I Heard You Like Registers              | Too much bloat code, but properly using conversion iterfaces though  |

