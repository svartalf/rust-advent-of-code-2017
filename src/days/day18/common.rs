use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum Value {
    Raw(isize),
    Register(char),
}

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Snd(Value),
    Set(Value, Value),
    Add(Value, Value),
    Mul(Value, Value),
    Mod(Value, Value),
    Rcv(Value),
    Jgz(Value, Value),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let command = parts.next();
        let mut x: Option<Value> = None;
        if let Some(x_raw) = parts.next() {
            if let Ok(value) = x_raw.parse::<isize>() {
                x = Some(Value::Raw(value));
            } else {
                x = Some(Value::Register(x_raw.chars().next().unwrap()));
            }
        }

        let mut y: Option<Value> = None;
        if let Some(y_raw) = parts.next() {
            if let Ok(value) = y_raw.parse::<isize>() {
                y = Some(Value::Raw(value));
            } else {
                y = Some(Value::Register(y_raw.chars().next().unwrap()));
            }
        }

        match command {
            Some("snd") => Ok(Instruction::Snd(x.unwrap())),
            Some("rcv") => Ok(Instruction::Rcv(x.unwrap())),
            Some("set") => Ok(Instruction::Set(x.unwrap(), y.unwrap())),
            Some("add") => Ok(Instruction::Add(x.unwrap(), y.unwrap())),
            Some("mul") => Ok(Instruction::Mul(x.unwrap(), y.unwrap())),
            Some("mod") => Ok(Instruction::Mod(x.unwrap(), y.unwrap())),
            Some("jgz") => Ok(Instruction::Jgz(x.unwrap(), y.unwrap())),
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
