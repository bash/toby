mod model;
mod context;

pub use self::model::*;
use self::context::{CommandError, JobContext};

use super::config::{Config, Project};
use super::telegram::{send_message, ParseMode, SendMessageParams};
use reqwest;
use std::time::Instant;
use std::slice::SliceConcatExt;
use std::io;
use std::fmt;
use std::sync::mpsc::Receiver;

macro status {
    ($fmt:expr) => {
        println!(concat!("[toby] ", $fmt));
    },
    ($fmt:expr, $($arg:tt)*) => {
        println!(concat!("[toby] ", $fmt), $($arg)*);
    }
}

#[derive(Debug)]
struct DeployStatus {
    duration: u64,
}

#[derive(Debug)]
enum DeployError {
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

fn deploy_project(job: &Job, project: &Project) -> Result<DeployStatus, DeployError> {
    let now = Instant::now();
    let project_name = job.project();
    let context = match JobContext::new() {
        Ok(context) => context,
        Err(err) => {
            status!("Failed to create context for {}: {}", project_name, err);
            return Err(DeployError::ContextError(err));
        }
    };

    status!(
        "Building project {} with context {} triggered by {}",
        project_name,
        context,
        job.trigger(),
    );

    for script in project.scripts() {
        let command = script.command();

        status!("Running command: {}", command.join(" "));

        let status = context.run_command(command);

        if let Err(err) = status {
            status!("{}", err);

            if !script.allow_failure() {
                status!("Unexpected failure. Cancelling deploy.");
                return Err(DeployError::CommandError(err));
            }
        }
    }

    Ok(DeployStatus {
        duration: now.elapsed().as_secs(),
    })
}

pub fn start_worker(config: Config, receiver: Receiver<Job>) {
    let client = reqwest::Client::new();
    let projects = config.projects();

    for job in receiver {
        let project_name = job.project();

        match projects.get(project_name) {
            Some(project) => {
                let deploy_result = deploy_project(&job, project);
                let telegram = config.main().telegram();

                if let Some(ref telegram) = *telegram {
                    let message = match deploy_result {
                        Ok(DeployStatus { duration }) => format!(
                            "✅ Deploy for project *{}* completed successfully after {}s, triggered by {}.",
                            project_name, duration, job.trigger(),
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
