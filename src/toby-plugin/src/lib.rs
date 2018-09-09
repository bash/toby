use self::job::Hook as JobHook;

pub mod job;

pub trait Registry {
    fn register_job_hook(&mut self, hook: Box<dyn JobHook>);
}
