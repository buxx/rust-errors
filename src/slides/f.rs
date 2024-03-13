#![allow(warnings, unused)]

// use `From` TRAIT TO CONVERT ERROR TYPES

struct InternalError(String);

// Here we define how a `io::Error` can become a `InternalError`
impl From<io::Error> for InternalError {
    fn from(value: io::Error) -> Self {
        Self(value.to_string())
    }
}

use std::{fs, io};

fn main() -> Result<(), InternalError> {
    // Automatic conversion done here
    let file_content = fs::read_to_string("todo.txt")?;
    todo!()
}
