use std::fmt;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Job {
    pub project: String,
    pub trigger: Trigger,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum Trigger {
    Webhook { token: String },
    Cli,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Outcome {
    Failure,
    Success,
}

impl Trigger {
    pub fn name(&self) -> &str {
        match self {
            Trigger::Webhook { .. } => "webhook",
            Trigger::Cli => "cli",
        }
    }
}

impl fmt::Display for Trigger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Trigger::Webhook { ref token } => write!(f, "webhook ({})", token),
            Trigger::Cli => write!(f, "cli"),
        }
    }
}
