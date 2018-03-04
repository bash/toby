use crate::config::Project;
use crate::fs::get_job_log;
use crate::worker::Job;
use std::borrow::{Borrow, Cow};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::Write;
use std::process::{Command, ExitStatus, Stdio};
use std::slice::SliceConcatExt;
use tempdir::TempDir;

const UNKNOWN_EXIT_STATUS: i32 = -1;

#[derive(Debug)]
pub enum CommandError {
    ExitStatus(ExitStatus),
    Io(io::Error),
}

#[derive(Debug)]
pub struct JobContext<'a> {
    current_dir: TempDir,
    job: &'a Job,
    environment: HashMap<&'a str, Cow<'a, str>>,
    log_file: File,
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
                status.code().unwrap_or(UNKNOWN_EXIT_STATUS)
            ),
            CommandError::Io(ref err) => write!(f, "Command failed: {}", err),
        }
    }
}

impl<'a> fmt::Display for JobContext<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (key, value) in &self.environment {
            writeln!(f, "  {}={}", key, value)?;
        }

        write!(f, "  PWD={}", self.current_dir.path().to_string_lossy())?;

        Ok(())
    }
}

impl<'a> JobContext<'a> {
    pub fn new(job: &'a Job, project: &'a Project) -> io::Result<Self> {
        let current_dir = TempDir::new("toby-job")?;

        let mut environment: HashMap<&'a str, Cow<'a, str>> = project
            .environment
            .iter()
            .map(|(key, value)| (key.as_str(), Cow::Borrowed(value.as_str())))
            .collect();

        environment.insert("TOBY_JOB_ID", job.id.to_string().into());
        environment.insert("TOBY_JOB_TRIGGER", job.trigger.name().into());

        let log_file = get_job_log(&job.project, job.id)?;

        Ok(Self {
            current_dir,
            job,
            environment,
            log_file,
        })
    }

    pub fn run_command<S>(&mut self, command: &[S]) -> Result<(), CommandError>
    where
        S: Borrow<str> + AsRef<OsStr>,
    {
        writeln!(
            self.log_file,
            "[toby] Running command {}",
            command.join(" ")
        )?;

        let mut cmd = Command::new(&command[0]);

        cmd.args(&command[1..])
            .current_dir(&self.current_dir)
            .stdout(Stdio::from(self.log_file.try_clone()?))
            .stderr(Stdio::from(self.log_file.try_clone()?));

        for (key, value) in &self.environment {
            cmd.env(key, value.as_ref());
        }

        let status = cmd.status()?;

        if status.success() {
            return Ok(());
        }

        Err(CommandError::ExitStatus(status))
    }
}
