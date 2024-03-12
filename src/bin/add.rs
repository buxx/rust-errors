use std::{
    env,
    fmt::{self, Debug, Display},
    fs::OpenOptions,
    io::{self, Write},
};

const FILE_NAME: &str = "todo.txt";
const FORBIDDEN: &[&str] = &["learn php"];

//
// ERRORS DECLARATIONS
//

enum AddError {
    UserInput(UserInputError),
    Internal(InternalError),
}

impl Display for AddError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AddError::UserInput(err) => f.write_fmt(format_args!("🤔 User input error: {err}")),
            AddError::Internal(err) => f.write_fmt(format_args!("💥 Internal error: {err}")),
        }
    }
}

impl From<UserInputError> for AddError {
    fn from(value: UserInputError) -> Self {
        AddError::UserInput(value)
    }
}

impl From<InternalError> for AddError {
    fn from(value: InternalError) -> Self {
        AddError::Internal(value)
    }
}

// This impl ensure main function to display the Display impl instead Debug
impl Debug for AddError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}

#[derive(Debug)]
enum UserInputError {
    ForbiddenValue(String),
}

impl Display for UserInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ForbiddenValue(value) => {
                f.write_fmt(format_args!("🚫 Forbidden value: '{value}'"))
            }
        }
    }
}

#[derive(Debug)]
enum InternalError {
    Io(io::Error),
}

impl From<io::Error> for InternalError {
    fn from(value: io::Error) -> Self {
        InternalError::Io(value)
    }
}

impl Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InternalError::Io(err) => f.write_fmt(format_args!("💾 Disk error: '{err}'")),
        }
    }
}

//
// BUSINESS CODE
//

fn check_user_input(args: &[String]) -> Result<(), UserInputError> {
    for arg in args {
        for forbidden in FORBIDDEN {
            if arg.to_ascii_lowercase().contains(forbidden) {
                return Err(UserInputError::ForbiddenValue(forbidden.to_string()));
            }
        }
    }

    Ok(())
}

// AddError will be displayed through the `impl Display`
fn main() -> Result<(), AddError> {
    let args: Vec<String> = env::args().skip(1).collect();
    // check_user_input return a `UserInputError` which is automatically
    // qualified into `AddError` by the `?`
    check_user_input(&args)?;

    let mut file = OpenOptions::new()
        .append(true)
        .open(FILE_NAME)
        // Qualify `io::Error` into ou business context error
        // which is itself qualified into `AddError` by the `?`
        .map_err(InternalError::from)?;

    for arg in args {
        writeln!(file, "[ ] {}", arg)
            // Qualify `io::Error` into ou business context error
            // which is itself qualified into `AddError` by the `?`
            .map_err(InternalError::from)?;
    }

    Ok(())
}
