#![allow(warnings, unused)]
struct InternalError(String);

use std::{fs, io};

// Note main expect a `InternalError` now
fn main() -> Result<(), InternalError> {
    let file_content = fs::read_to_string("todo.txt")
        // Map io::Error into InternalError
        .map_err(|error| InternalError(error.to_string()))?;
    todo!()
}
