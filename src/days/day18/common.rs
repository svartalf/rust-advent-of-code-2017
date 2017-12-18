use std::str::FromStr;

#[derive(Debug)]
pub enum Value {
    Raw(isize),
    Register(char),
}

#[derive(Debug)]
pub enum Instruction {
    Snd(char),
    Set(char, Value),
    Add(char, Value),
    Mul(char, Value),
    Mod(char, Value),
    Rcv(char),
    Jgz(char, Value),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let command = parts.next();
        // X param is always a char
        let x = parts.next().unwrap().chars().next().unwrap();
        let mut y: Option<Value> = None;
        if let Some(y_raw) = parts.next() {
            if let Ok(value) = y_raw.parse::<isize>() {
                y = Some(Value::Raw(value));
            } else {
                y = Some(Value::Register(y_raw.chars().next().unwrap()));
            }
        }

        match command {
            Some("snd") => Ok(Instruction::Snd(x)),
            Some("rcv") => Ok(Instruction::Rcv(x)),
            Some("set") => Ok(Instruction::Set(x, y.unwrap())),
            Some("add") => Ok(Instruction::Add(x, y.unwrap())),
            Some("mul") => Ok(Instruction::Mul(x, y.unwrap())),
            Some("mod") => Ok(Instruction::Mod(x, y.unwrap())),
            Some("jgz") => Ok(Instruction::Jgz(x, y.unwrap())),
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
