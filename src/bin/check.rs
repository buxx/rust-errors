use std::{
    env,
    fmt::{self, Debug},
    fs::OpenOptions,
    io,
};

use thiserror::Error;

const FILE_NAME: &str = "todo.txt";

#[derive(Error)]
enum CheckError {
    #[error("🤔 User input error: {0}")]
    UserInput(#[from] UserInputError),
    #[error("💥 Internal error: {0}")]
    Internal(#[from] InternalError),
}

// This impl ensure main function to display the Display impl instead Debug
impl Debug for CheckError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}

#[derive(Debug, Error)]
enum UserInputError {
    #[error("❓ Unknown task '{0}'")]
    Unknown(String),
}

#[derive(Debug, Error)]
enum InternalError {
    #[error("💾 Disk error: {0}")]
    Io(#[from] io::Error),
}

fn main() -> Result<(), CheckError> {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut file = OpenOptions::new()
        .append(true)
        .open(FILE_NAME)
        .map_err(InternalError::from)?;

    for arg in args {
        // TODO search line and check [x]
    }

    Ok(())
}
