use std::error;
use std::fmt;
use std::convert;
use std::num::ParseIntError;
use std::option::NoneError;
use std::collections::HashMap;


#[derive(Debug)]
pub struct ParseError();

impl error::Error for ParseError {
    fn description(&self) -> &str {
        "Failed to parse line"
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl convert::From<ParseIntError> for ParseError {
    fn from(_: ParseIntError) -> Self {
        ParseError()
    }
}

impl convert::From<NoneError> for ParseError {
    fn from(_: NoneError) -> Self {
        ParseError()
    }
}


#[derive(Debug)]
pub enum Cmp {
    Gt(isize),
    Gte(isize),
    Eq(isize),
    Lt(isize),
    Lte(isize),
    Ne(isize)
}

impl Cmp {
    pub fn new(op: &str, value: &str) -> Result<Cmp, ParseError> {
        let parsed_value = value.parse::<isize>()?;
        match op {
            ">" => Ok(Cmp::Gt(parsed_value)),
            ">=" => Ok(Cmp::Gte(parsed_value)),
            "==" => Ok(Cmp::Eq(parsed_value)),
            "<" => Ok(Cmp::Lt(parsed_value)),
            "<=" => Ok(Cmp::Lte(parsed_value)),
            "!=" => Ok(Cmp::Ne(parsed_value)),
            _ => Err(ParseError())
        }
    }

    pub fn cmp(&self, value: &isize) -> bool {
        match self {
            &Cmp::Gt(c) => *value > c,
            &Cmp::Gte(c) => *value >= c,
            &Cmp::Eq(c) => *value == c,
            &Cmp::Lt(c) => *value < c,
            &Cmp::Lte(c) => *value <= c,
            &Cmp::Ne(c) => *value != c,
        }
    }
}


#[derive(Debug)]
pub enum Op {
    Inc(isize),
    Dec(isize),
}

impl Op {
    pub fn new(op: &str, value: &str) -> Result<Op, ParseError> {
        let parsed_value = value.parse::<isize>()?;
        match op {
            "inc" => Ok(Op::Inc(parsed_value)),
            "dec" => Ok(Op::Dec(parsed_value)),
            _ => Err(ParseError()),
        }
    }

    pub fn op(&self, value: &isize) -> isize {
        match self {
            &Op::Inc(v) => value + v,
            &Op::Dec(v) => value - v,
        }
    }
}


#[derive(Debug)]
pub struct Line<'l> {
    pub register1: &'l str,
    pub op: Op,
    pub register2: &'l str,
    pub cmp: Cmp,
}

impl<'l> Line<'l> {
    pub fn from(line: &'l str) -> Result<Self, ParseError> {
        let mut split = line.split_whitespace();
        let register1 = split.next()?;
        let op = Op::new(
            split.next()?,
            split.next()?,
        )?;
        let register2 = split.nth(1)?;
        let cmp = Cmp::new(
            split.next()?,
            split.next()?,
        )?;

        Ok(Line {
            register1,
            op,
            register2,
            cmp,
        })
    }
}


pub struct Registry<'r>(HashMap<&'r str, isize>);

impl<'r> Registry<'r> {
    pub fn new() -> Self {
        Registry(HashMap::new())
    }

    pub fn data(&self) -> &HashMap<&str, isize> {
        &self.0
    }

    pub fn execute(&mut self, line: &'r Line) {
        let r1 = *self.0.get(&line.register1).unwrap_or(&0);
        let r2 = *self.0.get(&line.register2).unwrap_or(&0);

        if line.cmp.cmp(&r2) {
            let new_value = line.op.op(&r1);
            self.0.insert(line.register1, new_value);
        }
    }
}