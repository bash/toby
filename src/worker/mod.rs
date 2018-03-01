mod model;
mod context;

use self::context::{CommandError, JobContext};
pub use self::model::*;

use crate::config::{Config, Project};
use crate::fs::get_job_archive_file;
use crate::status;
use crate::telegram::{send_message, ParseMode, SendMessageParams};
use reqwest;
use std::fmt;
use std::io;
use std::io::Write;
use std::slice::SliceConcatExt;
use std::time::{SystemTime, UNIX_EPOCH};
use toml;

fn now() -> u64 {
    let sys_time = SystemTime::now();

    sys_time
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_secs()
}

#[derive(Debug)]
enum Error {
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

    fn run(&self) -> Result<(), Error> {
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

    fn run_scripts(&self) -> Result<(), Error> {
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

    fn archive_job(&self, started_at: u64, successful: bool) -> Result<(), Error> {
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
    let client = reqwest::Client::new();
    let projects = &config.projects;

    for job in receiver {
        let project_name = &job.project;

        match projects.get(project_name) {
            Some(project) => {
                let runner = JobRunner::new(&job, project);
                let job_result = runner.run();

                match job_result {
                    Ok(_) => status!("Job finished successfully"),
                    Err(ref err) => status!("{}", err),
                };

                let telegram = &config.main.telegram;

                if let Some(ref telegram) = *telegram {
                    let message = match job_result {
                        Ok(_) => format!(
                            "✅ Deploy for project *{}* completed successfully, triggered by {}.",
                            project_name, job.trigger,
                        ),
                        Err(err) => format!(
                            "⚠️ Deploy for project *{}* failed.\n```\n{}\n```",
                            project_name, err
                        ),
                    };

                    let params = SendMessageParams {
                        chat_id: &telegram.chat_id,
                        text: &message,
                        parse_mode: Some(ParseMode::Markdown),
                    };

                    let result = send_message(&client, &telegram.token, &params);

                    if let Err(err) = result {
                        status!("Unable to send telegram message: {}", err);
                    }
                }
            }
            None => status!("Project {} does not exist", project_name),
        }
    }
}
