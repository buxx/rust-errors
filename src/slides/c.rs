#![allow(warnings, unused)]
use std::{fs, io};

// DEAL WITH `Result` TYPE

fn main() -> Result<(), io::Error> {
    match fs::read_to_string("todo.txt") {
        Result::Ok(file_content) => todo!(),
        Result::Err(an_error) => return Result::Err(an_error),
    }
    todo!()
}
