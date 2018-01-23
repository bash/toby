use std::fmt;

#[derive(Debug)]
pub struct Job {
    project: String,
    trigger: JobTrigger,
}

#[derive(Debug)]
pub enum JobTrigger {
    Webhook { token: String },
    Telegram { username: String },
}

impl fmt::Display for JobTrigger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            JobTrigger::Webhook { ref token } => write!(f, "webhook with token {}", token),
            JobTrigger::Telegram { ref username } => write!(f, "telegram user {}", username),
        }
    }
}

impl Job {
    pub fn new<S: Into<String>>(project: S, trigger: JobTrigger) -> Self {
        let project = project.into();

        Job { project, trigger }
    }

    pub fn project(&self) -> &str {
        &self.project
    }

    pub fn trigger(&self) -> &JobTrigger {
        &self.trigger
    }
}
