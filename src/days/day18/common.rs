use std::str::FromStr;

#[derive(Debug)]
pub enum Instruction {
    Snd(char),
    Set(char, isize),
    Add(char, isize),
    Mul(char, char),
    Mod(char, isize),
    Rcv(char),
    Jgz(char, isize),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        match parts.next() {
            Some("snd") => Ok(Instruction::Snd(
                parts.next().unwrap().chars().next().unwrap(),
            )),
            Some("set") => Ok(Instruction::Set(
                parts.next().unwrap().chars().next().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )),
            Some("add") => Ok(Instruction::Add(
                parts.next().unwrap().chars().next().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )),
            Some("mul") => Ok(Instruction::Mul(
                parts.next().unwrap().chars().next().unwrap(),
                parts.next().unwrap().chars().next().unwrap(),
            )),
            Some("mod") => Ok(Instruction::Mod(
                parts.next().unwrap().chars().next().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )),
            Some("rcv") => Ok(Instruction::Rcv(
                parts.next().unwrap().chars().next().unwrap(),
            )),
            Some("jgz") => Ok(Instruction::Jgz(
                parts.next().unwrap().chars().next().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )),
            _ => Err(()),
        }
    }
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::Instruction;

    #[test]
    fn day18_common_test1() {
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

        for line in input.lines() {
            Instruction::from_str(line).unwrap();
        }
    }
}
