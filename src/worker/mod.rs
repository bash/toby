mod model;
mod context;
mod hook;
mod error;

pub use self::model::*;
use self::context::JobContext;
use self::error::{DeployError, DeployStatus};
use self::hook::{Hook, TelegramHook};

use super::config::{Config, Project};
use super::status;
use std::time::Instant;
use std::slice::SliceConcatExt;
use std::sync::mpsc::Receiver;

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
    let telegram_hook = TelegramHook::with_config(&config);
    let mut hooks: Vec<&Hook> = Vec::new();

    if let Some(ref telegram_hook) = telegram_hook {
        hooks.push(telegram_hook);
    }

    let projects = config.projects();

    for job in receiver {
        let project_name = job.project();

        match projects.get(project_name) {
            Some(project) => {
                for hook in &hooks {
                    hook.before_deploy(&job);
                }

                let deploy_result = deploy_project(&job, project);

                for hook in &hooks {
                    hook.after_deploy(&job, &deploy_result);
                }
            }
            None => status!("Project {} does not exist", project_name),
        }
    }
}
