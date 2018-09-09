use std::error::Error;
use toby_plugin::{Job, JobHook, JobOutcome, Registry};

#[derive(Debug)]
struct Telegram;

impl JobHook for Telegram {
    fn before_job(&self, _job: Job<'_>) -> Result<(), Box<dyn Error>> {
        unimplemented!();
    }

    fn after_job(&self, _job: Job<'_>, _outcome: JobOutcome) -> Result<(), Box<dyn Error>> {
        unimplemented!();
    }
}

pub fn plugin_registrar(registry: &mut Registry) {
    registry.register_job_hook(Box::new(Telegram));
}
