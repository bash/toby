use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::{error, fmt, io};

pub struct Command<'a> {
    pub command: Vec<&'a OsStr>,
    pub environment: &'a HashMap<&'a OsStr, OsString>,
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    ExitStatus(Option<i32>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(ref err) => write!(f, "Command failed: {}", err),
            Error::ExitStatus(Some(code)) => write!(f, "Command failed with exit code: {}", code),
            Error::ExitStatus(None) => write!(f, "Command failed"),
        }
    }
}

impl error::Error for Error {}

pub trait CommandExecutor {
    fn execute_command(&self, command: Command<'_>) -> Result<(), Error>;
}
