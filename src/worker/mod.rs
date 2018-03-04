mod model;
mod context;
mod hook;

use self::context::{CommandError, JobContext};
pub use self::model::*;

use self::hook::{Hook, Hooks};
use crate::config::{Config, Project};
use crate::fs::{get_job_archive_file, get_telegram_chat_id};
use crate::status;
use crate::time::now;
use std::fmt;
use std::io;
use std::io::Write;
use std::slice::SliceConcatExt;
use toml;

pub(crate) type JobResult = Result<(), Error>;

#[derive(Debug)]
pub(crate) enum Error {
    Context(io::Error),
    Command(CommandError),
    Archive(io::Error),
}

#[derive(Debug)]
struct JobRunner<'a> {
    job: &'a Job,
    project: &'a Project,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Context(ref err) => write!(f, "Unable to create context: {}", err),
            Error::Command(ref err) => write!(f, "{}", err),
            Error::Archive(ref err) => write!(f, "Unable to archive job: {}", err),
        }
    }
}

impl<'a> JobRunner<'a> {
    fn new(job: &'a Job, project: &'a Project) -> Self {
        JobRunner { job, project }
    }

    fn run(&self) -> JobResult {
        let started_at = now();
        let project_name = &self.job.project;

        status!(
            "Starting job #{} for {}, triggered by {}",
            self.job.id,
            project_name,
            self.job.trigger
        );

        let result = self.run_scripts();

        self.archive_job(started_at, result.is_ok())?;

        result
    }

    fn run_scripts(&self) -> JobResult {
        let mut context = JobContext::new(self.job, self.project).map_err(Error::Context)?;

        println!("{}", context);

        for script in &self.project.scripts {
            let command = &script.command;

            status!("Running command: {}", command.join(" "));

            let status = context.run_command(command);

            if let Err(ref err) = status {
                status!("{}", err);
            }

            status.map_err(Error::Command).or_else(|err| {
                if script.allow_failure {
                    Ok(())
                } else {
                    Err(err)
                }
            })?;
        }

        Ok(())
    }

    fn archive_job(&self, started_at: u64, successful: bool) -> JobResult {
        let job = &self.job;

        let file = get_job_archive_file(&job.project, job.id).map_err(Error::Archive)?;
        let mut buf_writer = io::BufWriter::new(file);
        let archived_job = self.job.archive(started_at, successful);
        let archived_job_str = toml::to_string(&archived_job).expect("unable to serialize job");

        buf_writer
            .write_all(archived_job_str.as_bytes())
            .map_err(Error::Archive)?;

        Ok(())
    }
}

pub fn start_worker(config: &Config, receiver: &WorkerReceiver) {
    let projects = &config.projects;

    let telegram_chat_id = get_telegram_chat_id().expect("Unable to read telegram chat id");
    let hooks = Hooks::from_config(config, telegram_chat_id);

    for job in receiver {
        let project_name = &job.project;

        match projects.get(project_name) {
            Some(project) => {
                let runner = JobRunner::new(&job, project);

                hooks.before_job(&job);

                let job_result = runner.run();

                match job_result {
                    Ok(_) => status!("Job finished successfully"),
                    Err(ref err) => status!("{}", err),
                };

                hooks.after_job(&job, &job_result);
            }
            None => status!("Project {} does not exist", project_name),
        }
    }
}
