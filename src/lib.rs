#![feature(test)]

#[cfg(feature="nightly")]
extern crate test;

use std::io::{Read, Result};
use std::fs::File;
use std::path::Path;
use std::convert::AsRef;

pub mod days;

pub fn read<T>(path: T) -> Result<String> where T: AsRef<Path> {
    let mut f = File::open(path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    Ok(contents.trim().to_string())
}
