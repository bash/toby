use std::fmt;

pub type JobId = u64;

#[derive(Debug)]
pub struct Job {
    pub id: JobId,
    pub project: String,
    pub trigger: JobTrigger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArchivedJob {
    pub started_at: u64,
    pub successful: bool,
    pub trigger: ArchivedJobTrigger,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum JobTrigger {
    Webhook { token: String },
    Cli,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ArchivedJobTrigger {
    Webhook { token: String },
    Cli,
}

impl JobTrigger {
    pub fn name(&self) -> &str {
        match *self {
            JobTrigger::Webhook { .. } => "webhook",
            JobTrigger::Cli => "cli",
        }
    }
}

impl From<JobTrigger> for ArchivedJobTrigger {
    fn from(trigger: JobTrigger) -> Self {
        match trigger {
            JobTrigger::Cli => ArchivedJobTrigger::Cli,
            JobTrigger::Webhook { token } => ArchivedJobTrigger::Webhook { token },
        }
    }
}

impl fmt::Display for JobTrigger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            JobTrigger::Webhook { ref token } => write!(f, "webhook ({})", token),
            JobTrigger::Cli => write!(f, "cli"),
        }
    }
}

impl Job {
    pub fn archive(&self, started_at: u64, successful: bool) -> ArchivedJob {
        ArchivedJob {
            trigger: self.trigger.clone().into(),
            started_at,
            successful,
        }
    }
}
