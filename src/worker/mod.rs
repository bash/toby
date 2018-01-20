mod model;
mod context;

pub use self::model::*;
use self::context::{CommandError, JobContext};

use super::config::{get_config, get_projects, Project};
use super::ipc::{Receiver, Server};
use super::telegram::{send_message, ParseMode, SendMessageParams};
use std::process;
use reqwest;
use std::time::Instant;
use std::slice::SliceConcatExt;
use std::io;
use std::fmt;

macro status {
    ($fmt:expr) => {
        println!(concat!("[toby] ", $fmt));
    },
    ($fmt:expr, $($arg:tt)*) => {
        println!(concat!("[toby] ", $fmt), $($arg)*);
    }
}

macro unwrap_config {
    ($config:expr) => {
        match $config {
            Ok(config) => config,
            Err(err) => {
                eprintln!("{}", err);
                process::exit(1);
            }
        }
    };
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

fn deploy_project(project_name: &str, project: &Project) -> Result<DeployStatus, DeployError> {
    let now = Instant::now();
    let context = match JobContext::new() {
        Ok(context) => context,
        Err(err) => {
            status!("Failed to create context for {}: {}", project_name, err);
            return Err(DeployError::ContextError(err));
        }
    };

    status!("Building project {} {}", project_name, context);

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

pub fn start_worker() {
    let projects = unwrap_config!(get_projects());
    let config = unwrap_config!(get_config());

    let server = Server::new().unwrap();
    let receiver: Receiver<Job> = server.connect().unwrap();
    let client = reqwest::Client::new();

    for job in receiver {
        let job = job.unwrap();
        let project_name = job.project();

        match projects.get(project_name) {
            Some(project) => {
                let deploy_result = deploy_project(project_name, project);
                let telegram = config.telegram();

                if let Some(ref telegram) = *telegram {
                    let message = match deploy_result {
                        Ok(DeployStatus { duration }) => format!(
                            "✅ Deploy for project *{}* completed successfully after {}s.",
                            project_name, duration
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
