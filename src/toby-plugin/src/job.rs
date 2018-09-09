use std::error::Error;
pub use toby_core::job::{Job, Outcome};

pub trait Hook {
    fn before_job(&self, job: &'_ Job) -> Result<(), Box<dyn Error>>;
    fn after_job(&self, job: &'_ Job, outcome: Outcome) -> Result<(), Box<dyn Error>>;
}
