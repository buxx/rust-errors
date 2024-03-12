#![allow(warnings, unused)]
use std::{fs, io};

fn main() -> Result<(), io::Error> {
    match fs::read_to_string("todo.txt") {
        Ok(file_content) => todo!(),
        Err(an_error) => return Err(an_error),
    }
    todo!()
}
