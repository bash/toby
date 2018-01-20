mod model;

pub use self::model::*;

use super::config::{get_config, get_projects, Project};
use super::ipc::{Receiver, Server};
use super::telegram::{send_message, ParseMode, SendMessageParams};
use std::process::{self, Command};
use reqwest;
use std::time::Instant;

macro status {
    ($fmt:expr, $($arg:tt)*) => {
        println!(concat!("[toby] ", $fmt), $($arg)*)
    };
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

struct DeployStatus {
    duration: u64,
}

fn deploy_project(project_name: &str, project: &Project) -> Result<DeployStatus, ()> {
    let now = Instant::now();

    status!("Building project {}", project_name);

    for script in project.scripts() {
        let command = script.command();

        status!("Running {:?}", command);

        let status = Command::new(&command[0]).args(&command[1..]).status();

        let failed = match status {
            Ok(status) => !status.success(),
            Err(err) => {
                status!("Execution failed: {}", err);
                true
            }
        };

        if failed && !script.allow_failure() {
            return Err(());
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
                        Err(_) => format!("⚠️ Deploy for project *{}* failed.", project_name),
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
