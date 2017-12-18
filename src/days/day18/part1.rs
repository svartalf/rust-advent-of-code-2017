use std::collections::HashMap;

use super::common::{Instruction, Value};

fn get_value(map: &HashMap<char, isize>, value: &Value) -> isize {
    match value {
        &Value::Raw(ref val) => *val,
        &Value::Register(register) => *map.get(&register).unwrap_or(&0),
    }
}

pub fn parse(data: &str) -> isize {
    let instructions: Vec<_> = data.lines()
        .map(|line| line.parse::<Instruction>().unwrap())
        .collect();
    let mut map: HashMap<char, isize> = "abcdefghijklmnopqrstuvwxyz".chars()
        .map(|chr| (chr, 0)).collect();
    let mut last_freq: Option<isize> = None;
    let mut idx = 0;

    loop {
        if idx > instructions.len() {
            break;
        }

        match instructions[idx] {
            Instruction::Add(register, ref value) => {
                let val = get_value(&map, &value);
                let entry = map.entry(register).or_insert(0);
                (*entry) += val;

                idx += 1;
            }
            Instruction::Mul(register1, ref value) => {
                let val = get_value(&map, &value);
                let entry = map.entry(register1).or_insert(0);
                (*entry) *= val;

                idx += 1;
            },
            Instruction::Mod(register, ref value) => {
                let val = get_value(&map, &value);
                let entry = map.entry(register).or_insert(0);
                (*entry) = (*entry % val + val) % val;

                idx += 1;
            },
            Instruction::Set(register, ref value) => {
                let val = get_value(&map, &value);
                let entry = map.entry(register).or_insert(0);
                (*entry) = val;

                idx += 1;
            },
            Instruction::Jgz(register, ref value) => {
                let cmp = *map.get(&register).unwrap_or(&0);
                if cmp > 0 {
                    let val = get_value(&map, &value);
                    if val > 0 {
                        idx += val as usize;
                    } else {
                        idx -= val.abs() as usize;
                    }
                } else {
                    idx += 1;
                }
            },
            Instruction::Snd(register) => {
                let freq = map.get(&register).unwrap_or(&0);
                last_freq = Some(*freq);

                idx += 1;
            },
            Instruction::Rcv(register) => {
                let entry = map.entry(register).or_insert(0);
                if *entry != 0 {
                    return last_freq.unwrap();
                }

                idx += 1;
            },
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
