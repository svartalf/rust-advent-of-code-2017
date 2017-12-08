use std::collections::HashMap;

type Registry = HashMap<String, isize>;

#[derive(Debug)]
struct Line {
    pub register1: String,
    pub op: Op,
    pub register2: String,
    pub cmp: Ordering,
}

// TODO: Not Ordering
#[derive(Debug)]
enum Ordering {
    Gt(isize),
    Gte(isize),
    Eq(isize),
    Lt(isize),
    Lte(isize),
    Ne(isize)
}

impl Ordering {
    pub fn new(op: &str, value: &str) -> Result<Ordering, ()> {
        let parsed_value = value.parse::<isize>().unwrap();
        match op {
            ">" => Ok(Ordering::Gt(parsed_value)),
            ">=" => Ok(Ordering::Gte(parsed_value)),
            "==" => Ok(Ordering::Eq(parsed_value)),
            "<" => Ok(Ordering::Lt(parsed_value)),
            "<=" => Ok(Ordering::Lte(parsed_value)),
            "!=" => Ok(Ordering::Ne(parsed_value)),
            _ => Err(())
        }
    }

    pub fn cmp(&self, value: &isize) -> bool {
        match self {
            &Ordering::Gt(c) => *value > c,
            &Ordering::Gte(c) => *value >= c,
            &Ordering::Eq(c) => *value == c,
            &Ordering::Lt(c) => *value < c,
            &Ordering::Lte(c) => *value <= c,
            &Ordering::Ne(c) => *value != c,
        }
    }
}


#[derive(Debug)]
enum Op {
    Inc(isize),
    Dec(isize),
}

impl Op {
    pub fn new(op: &str, value: &str) -> Result<Op, ()> {
        let parsed_value = value.parse::<isize>().unwrap();
        match op {
            "inc" => Ok(Op::Inc(parsed_value)),
            "dec" => Ok(Op::Dec(parsed_value)),
            _ => Err(()),
        }
    }

    pub fn op(&self, value: &isize) -> isize {
        match self {
            &Op::Inc(v) => value + v,
            &Op::Dec(v) => value - v,
        }
    }
}


pub fn parse(data: &str) -> isize {
    let mut registry = Registry::new();
    let mut total_max: isize = 0;

    data.lines().map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let cmp = Ordering::new(parts[5], parts[6]).unwrap();
        let op = Op::new(parts[1], parts[2]).unwrap();
        Line {
            register1: parts[0].to_string(),
            cmp: cmp,
            register2: parts[4].to_string(),
            op: op,
        }
    }).for_each(|line| {
        let r1 = *registry.get(&line.register1).unwrap_or(&0);
        let r2 = *registry.get(&line.register2).unwrap_or(&0);

        // part2
        if r1 > total_max {
            total_max = r1;
        }
        if r2 > total_max {
            total_max = r2;
        }
        //

        if line.cmp.cmp(&r2) {
            let new_value = line.op.op(&r1);
            registry.insert(line.register1.clone(), new_value);
        }
    });

    println!("total max: {}", total_max);

    *registry.values().max().unwrap_or(&0)
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn day08_part1_test1() {
        let data = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10
";

        assert_eq!(1, parse(data));
    }
}
