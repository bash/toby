use crate::command_executor::CommandExecutor;
use crate::os_path::OsPath;
use toby_core::config::Project;
use toby_core::job::Job;
use toby_plugin::job::Hook;

pub type CommandExecutorFactory = dyn Fn() -> Box<dyn CommandExecutor>;

#[allow(dead_code)]
pub struct JobRunner {
    command_executor_factory: Box<CommandExecutorFactory>,
    environment_builder: Box<dyn OsPath>,
    hooks: Vec<Box<dyn Hook>>,
}

impl JobRunner {
    pub fn run_job(&self, _job: Job, _project: &Project) {
        unimplemented!();
    }
}
