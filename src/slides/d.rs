#![allow(warnings, unused)]
use std::{fs, io};

// SIMPLIFY READING WITH `?`

fn main() -> Result<(), io::Error> {
    let file_content = fs::read_to_string("todo.txt")?;
    todo!()
}
