use std::{fmt, io};
use super::context::CommandError;

pub type DeployResult = Result<DeployStatus, DeployError>;

#[derive(Debug)]
pub struct DeployStatus {
    pub duration: u64,
}

#[derive(Debug)]
pub enum DeployError {
    ContextError(io::Error),
    CommandError(CommandError),
}

impl fmt::Display for DeployError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeployError::ContextError(ref err) => write!(f, "Unable to create context: {}", err),
            DeployError::CommandError(ref err) => write!(f, "{}", err),
        }
    }
}
