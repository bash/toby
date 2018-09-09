use std::error::Error;
use toby_plugin::job::{Hook, Job, Outcome};
use toby_plugin::Registry;

struct Telegram;

impl Hook for Telegram {
    fn before_job(&self, _job: &'_ Job) -> Result<(), Box<dyn Error>> {
        unimplemented!();
    }

    fn after_job(&self, _job: &'_ Job, _outcome: Outcome) -> Result<(), Box<dyn Error>> {
        unimplemented!();
    }
}

#[no_mangle]
pub fn plugin_registrar(registry: &mut dyn Registry) {
    registry.register_job_hook(Box::new(Telegram));
}
