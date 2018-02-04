use tempdir::TempDir;
use std::io;
use std::ffi::OsStr;
use std::process::{Command, ExitStatus, Stdio};
use std::fmt;
use std::io::Write;
use std::slice::SliceConcatExt;
use std::borrow::{Borrow, Cow};
use std::fs::File;
use crate::fs::get_job_log;
use crate::worker::Job;

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
    envs: Vec<(&'a str, Cow<'a, str>)>,
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
        for &(ref key, ref value) in &self.envs {
            writeln!(f, "  {}={}", key, value)?;
        }

        write!(f, "  PWD={}", self.current_dir.path().to_string_lossy())?;

        Ok(())
    }
}

impl<'a> JobContext<'a> {
    pub fn new(job: &'a Job) -> io::Result<Self> {
        let current_dir = TempDir::new("toby-job")?;

        let envs = vec![
            ("TOBY_JOB_ID", job.id.to_string().into()),
            ("TOBY_JOB_TRIGGER", job.trigger.name().into()),
        ];

        let log_file = get_job_log(&job.project, job.id)?;

        Ok(Self {
            current_dir,
            job,
            envs,
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

        for &(ref key, ref value) in &self.envs {
            cmd.env(key, value.as_ref());
        }

        let status = cmd.status()?;

        if status.success() {
            return Ok(());
        }

        Err(CommandError::ExitStatus(status))
    }
}
