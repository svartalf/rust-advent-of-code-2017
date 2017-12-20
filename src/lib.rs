#![feature(test)]
#![feature(try_trait)]

#[cfg(feature="nightly")]
extern crate test;

extern crate regex;

use std::io::{Read, Result};
use std::fs::File;
use std::path::Path;
use std::convert::AsRef;

pub mod days;

pub fn read<T>(path: T) -> Result<String> where T: AsRef<Path> {
    let mut f = File::open(path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn read_trim<T>(path: T) -> Result<String> where T: AsRef<Path> {
    let value = read(path)?;

    Ok(value.trim().to_string())
}
