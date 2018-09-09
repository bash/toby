use std::error::Error;
use toby_core::job::JobTrigger;

pub struct Job<'a> {
    pub project_name: &'a str,
    pub trigger: &'a JobTrigger,
}

pub enum JobOutcome {
    Failure,
    Success,
}

pub trait JobHook {
    fn before_job(&self, job: Job<'_>) -> Result<(), Box<dyn Error>>;
    fn after_job(&self, job: Job<'_>, outcome: JobOutcome) -> Result<(), Box<dyn Error>>;
}

#[derive(Default)]
pub struct Registry {
    job_hooks: Vec<Box<dyn JobHook>>,
}

impl Registry {
    pub fn register_job_hook(&mut self, hook: Box<dyn JobHook>) {
        self.job_hooks.push(hook);
    }

    pub fn consume(self) -> (Vec<Box<dyn JobHook>>,) {
        let Self { job_hooks } = self;

        (job_hooks,)
    }
}
