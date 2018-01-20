use tempdir::TempDir;
use std::io;
use std::ffi::OsStr;
use std::process::{Command, ExitStatus};
use std::fmt;

#[derive(Debug)]
pub enum CommandError {
    ExitStatus(ExitStatus),
    Io(io::Error),
}

#[derive(Debug)]
pub struct JobContext {
    current_dir: TempDir,
}

impl From<io::Error> for CommandError {
    fn from(err: io::Error) -> Self {
        CommandError::Io(err)
    }
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CommandError::ExitStatus(ref status) => write!(
                f,
                "Command failed with exit status: {}",
                status.code().unwrap_or(-1)
            ),
            CommandError::Io(ref err) => write!(f, "Command failed: {}", err),
        }
    }
}

impl fmt::Display for JobContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ pwd = {} }}",
            self.current_dir.path().to_string_lossy()
        )
    }
}

impl JobContext {
    pub fn new() -> io::Result<Self> {
        let current_dir = TempDir::new("toby-job")?;

        Ok(Self { current_dir })
    }

    pub fn run_command<S>(&self, command: &[S]) -> Result<(), CommandError>
    where
        S: AsRef<OsStr>,
    {
        let status = Command::new(&command[0])
            .args(&command[1..])
            .current_dir(&self.current_dir)
            .status()?;

        if status.success() {
            return Ok(());
        }

        Err(CommandError::ExitStatus(status))
    }
}
