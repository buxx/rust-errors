use std::{
    env,
    fmt::{self, Debug},
    fs::{File, OpenOptions},
    io::{self, Read, Write},
};

use thiserror::Error;

const FILE_NAME: &str = "todo.txt";

//
// ERRORS DECLARATIONS
//

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
    UnknownTask(String),
}

#[derive(Debug, Error)]
enum InternalError {
    #[error("💾 Disk error: {0}")]
    Io(#[from] io::Error),
}

//
// BUSINESS CODE
//

fn line_match(line: &str, arg: &str) -> bool {
    line.to_lowercase().contains(&arg.to_lowercase())
}

fn find_line(lines: &[&str], arg: &str) -> Option<String> {
    lines
        .iter()
        .find(|&line| line_match(line, arg))
        .map(|line| line.to_string())
}

fn check_user_input(lines: &[&str], args: &[String]) -> Result<(), UserInputError> {
    for arg in args {
        if find_line(lines, arg).is_none() {
            return Err(UserInputError::UnknownTask(arg.to_string()));
        }
    }

    Ok(())
}

fn main() -> Result<(), CheckError> {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut file_content = String::new();
    File::open(FILE_NAME)
        // Qualify `io::Error` into ou business context error
        // which is itself qualified into `AddError` by the `?`
        .map_err(InternalError::from)?
        .read_to_string(&mut file_content)
        // Qualify `io::Error` into ou business context error
        // which is itself qualified into `AddError` by the `?`
        .map_err(InternalError::from)?;
    let lines: Vec<&str> = file_content.lines().collect();

    // check_user_input return a `UserInputError` which is automatically
    // qualified into `CheckError` by the `?`
    check_user_input(&lines, &args)?;

    let mut new_lines: Vec<String> = vec![];
    for line in lines {
        for arg in &args {
            if line_match(line, arg) {
                let task = &line[4..];
                new_lines.push(format!("[x] {}", task));
            } else {
                new_lines.push(line.to_string())
            }
        }
    }

    OpenOptions::new()
        .write(true)
        .open(FILE_NAME)
        // Qualify `io::Error` into ou business context error
        // which is itself qualified into `AddError` by the `?`
        .map_err(InternalError::from)?
        .write_all(new_lines.join("\n").as_bytes())
        // Qualify `io::Error` into ou business context error
        // which is itself qualified into `AddError` by the `?`
        .map_err(InternalError::from)?;

    Ok(())
}
