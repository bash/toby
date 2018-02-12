mod model;
mod context;

pub use self::model::*;
use self::context::{CommandError, JobContext};

use super::config::{Config, Project, Script};
use super::telegram::{send_message, ParseMode, SendMessageParams};
use super::status;
use reqwest;
use std::slice::SliceConcatExt;
use std::io;
use std::fmt;

#[derive(Debug)]
enum JobError {
    ContextError(io::Error),
    CommandError(CommandError),
}

impl fmt::Display for JobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            JobError::ContextError(ref err) => write!(f, "Unable to create context: {}", err),
            JobError::CommandError(ref err) => write!(f, "{}", err),
        }
    }
}

fn run_job(job: &Job, project: &Project) -> Result<(), JobError> {
    let project_name = &job.project;

    status!(
        "Starting job #{} for {}, triggered by {}",
        job.id,
        project_name,
        job.trigger
    );

    let mut context = match JobContext::new(job, project) {
        Ok(context) => context,
        Err(err) => {
            status!("Unable to create context: {}", err);
            return Err(JobError::ContextError(err));
        }
    };

    println!("{}", context);

    let scripts_result = run_scripts(&mut context, project.scripts());

    // TODO: write archived job

    scripts_result
}

fn run_scripts(context: &mut JobContext, scripts: &[Script]) -> Result<(), JobError> {
    for script in scripts {
        let command = script.command();

        status!("Running command: {}", command.join(" "));

        let status = context.run_command(command);

        if let Err(err) = status {
            status!("{}", err);

            if !script.allow_failure() {
                status!("Unexpected failure. Cancelling job.");
                return Err(JobError::CommandError(err));
            }
        }
    }

    Ok(())
}

pub fn start_worker(config: Config, receiver: WorkerReceiver) {
    let client = reqwest::Client::new();
    let projects = config.projects();

    for job in receiver {
        let project_name = &job.project;

        match projects.get(project_name) {
            Some(project) => {
                let deploy_result = run_job(&job, project);
                let telegram = config.main().telegram();

                if let Some(ref telegram) = *telegram {
                    let message = match deploy_result {
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
                        chat_id: telegram.chat_id(),
                        text: &message,
                        parse_mode: Some(ParseMode::Markdown),
                    };

                    let result = send_message(&client, telegram.token(), &params);

                    if let Err(err) = result {
                        status!("Unable to send telegram message: {}", err);
                    }
                }
            }
            None => status!("Project {} does not exist", project_name),
        }
    }
}
