use std::sync::{mpsc, Arc, Barrier};
use std::time::Duration;
use std::collections::HashMap;
use std::thread;

use super::common::{Instruction, Value};

fn get_value(map: &HashMap<char, isize>, value: &Value) -> isize {
    match value {
        &Value::Raw(ref val) => *val,
        &Value::Register(register) => *map.get(&register).unwrap_or(&0),
    }
}

#[derive(Debug)]
enum Message {
    Stop,
    Value(isize),
}

fn run(program: isize, tx: mpsc::Sender<Message>, rx: mpsc::Receiver<Message>,
       instructions: Vec<Instruction>) -> (isize, usize) {
    let mut map: HashMap<char, isize> = "abcdefghijklmnopqrstuvwxyz".chars()
        .map(|chr| (chr, 0)).collect();
    map.insert('p', program);
    let mut idx = 0;
    let mut sends = 0usize;

    loop {
        if idx > instructions.len() {
            tx.send(Message::Stop).unwrap();
            break;
        }

        match instructions[idx] {
            Instruction::Add(source, ref value) => {
                let val = get_value(&map, &value);
                let register = match source {
                    Value::Register(reg) => reg,
                    _ => unreachable!(),
                };
                let entry = map.entry(register).or_insert(0);
                (*entry) += val;

                idx += 1;
            }
            Instruction::Mul(source, ref value) => {
                let val = get_value(&map, &value);
                let register = match source {
                    Value::Register(reg) => reg,
                    _ => unreachable!(),
                };
                let entry = map.entry(register).or_insert(0);
                (*entry) *= val;

                idx += 1;
            },
            Instruction::Mod(source, ref value) => {
                let val = get_value(&map, &value);
                let register = match source {
                    Value::Register(reg) => reg,
                    _ => unreachable!(),
                };
                let entry = map.entry(register).or_insert(0);
                (*entry) = (*entry % val + val) % val;

                idx += 1;
            },
            Instruction::Set(source, ref value) => {
                let val = get_value(&map, &value);
                let register = match source {
                    Value::Register(reg) => reg,
                    _ => unreachable!(),
                };
                let entry = map.entry(register).or_insert(0);
                (*entry) = val;

                idx += 1;
            },
            Instruction::Jgz(ref register, ref value) => {
                let cmp = match register {
                    &Value::Raw(val) => val,
                    &Value::Register(name) => *map.get(&name).unwrap(),
                };
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
            Instruction::Snd(source) => {
                let value = match source {
                    Value::Register(reg) => *map.get(&reg).unwrap(),
                    Value::Raw(val) => val,
                };
                tx.send(Message::Value(value)).unwrap();
                sends += 1;

                idx += 1;
            },
            Instruction::Rcv(source) => {
                let register = match source {
                    Value::Register(reg) => reg,
                    _ => unreachable!(),
                };
                match rx.recv_timeout(Duration::new(3, 0)) {
                    Ok(Message::Value(value)) => {
                        let entry = map.entry(register).or_insert(0);
                        (*entry) = value;
                    },
                    _ => break,
                }

                idx += 1;
            },
        }
    }

    (program, sends)
}

pub fn parse(data: &str) -> usize {
    let instructions: Vec<_> = data.lines()
        .map(|line| line.parse::<Instruction>().unwrap())
        .collect();

    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    let barrier = Arc::new(Barrier::new(2));

    let instructions2 = instructions.clone();
    let b1 = barrier.clone();
    let thread1 = thread::spawn(move || {
        b1.wait();
        run(0, tx1, rx2, instructions)
    });

    let b2 = barrier.clone();
    let thread2 = thread::spawn(move || {
        b2.wait();
        run(1, tx2, rx1, instructions2)
    });
    thread1.join().unwrap();
    let (_, sends) = thread2.join().unwrap();

    sends
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn day18_part2_test1() {
        let input = "snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d";
        assert_eq!(3, parse(input));
    }
}
