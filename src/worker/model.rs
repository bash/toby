use std::fmt;
use std::sync::mpsc::{Receiver, SyncSender};

pub type JobId = u64;
pub type WorkerSender = SyncSender<Job>;
pub type WorkerReceiver = Receiver<Job>;

#[derive(Debug)]
pub struct Job {
    pub id: JobId,
    pub project: String,
    pub trigger: JobTrigger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArchivedJob {
    pub trigger: JobTrigger,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum JobTrigger {
    Webhook { token: String },
    Telegram { username: String },
}

impl JobTrigger {
    pub fn name(&self) -> &str {
        match *self {
            JobTrigger::Webhook { .. } => "webhook",
            JobTrigger::Telegram { .. } => "telegram",
        }
    }
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
    pub fn archive(self) -> (JobId, String, ArchivedJob) {
        let Job {
            id,
            project,
            trigger,
        } = self;

        (id, project, ArchivedJob { trigger })
    }
}
