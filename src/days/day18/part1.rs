use std::collections::HashMap;

use super::common::Instruction;

pub fn parse(data: &str) -> usize {
    let mut map: HashMap<char, isize> = HashMap::new();
    for chr in "abcdefghijklmnopqrstuvwxyz".chars() {
        map.insert(chr, 0);
    }

    let instructions: Vec<_> = data.lines()
        .map(|line| line.parse::<Instruction>().unwrap())
        .collect();

    println!("{:?}", instructions);

    for instruction in instructions.iter() {
        match instruction {
            &Instruction::Add(register, value) => {
                let entry = map.entry(register).or_insert(0);
                (*entry) += value;
            }
            &Instruction::Mul(register1, register2) => {
                let value = *map.get(&register2).unwrap();
                let entry = map.entry(register1).or_insert(0);
                (*entry) *= value;
            },
            &Instruction::Mod(register, value) => {
                let entry = map.entry(register).or_insert(0);
                (*entry) = (*entry % value + value) % value;
            },
            &Instruction::Set(register, value) => {
                let entry = map.entry(register).or_insert(0);
                (*entry) = value;
            }
            _ => {}
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn day18_part1_test1() {
        let input = "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";
        assert_eq!(4, parse(input));
    }
}
