use super::JobResult;
use super::model::Job;
use crate::config::Config;
use crate::status;
use crate::telegram;
use reqwest;

#[derive(Debug)]
struct TelegramHook {
    api: telegram::Api,
    chat_id: String,
}

#[derive(Debug)]
pub(crate) struct Hooks {
    telegram: Option<TelegramHook>,
}

pub(crate) trait Hook {
    fn before_job(&self, job: &Job);
    fn after_job(&self, job: &Job, result: &JobResult);
}

impl Hooks {
    pub(crate) fn from_config(config: &Config) -> Self {
        let telegram = TelegramHook::from_config(config);

        Hooks { telegram }
    }
}

impl Hook for Hooks {
    fn before_job(&self, job: &Job) {
        if let Some(ref telegram) = self.telegram {
            telegram.before_job(job);
        }
    }

    fn after_job(&self, job: &Job, result: &JobResult) {
        if let Some(ref telegram) = self.telegram {
            telegram.after_job(job, result);
        }
    }
}

impl TelegramHook {
    fn from_config(config: &Config) -> Option<Self> {
        let telegram = config.main.telegram.as_ref();

        telegram.map(|ref telegram| TelegramHook {
            api: telegram::Api::new(reqwest::Client::new(), &telegram.token),
            chat_id: telegram.chat_id.to_string(),
        })
    }
}

impl Hook for TelegramHook {
    fn before_job(&self, job: &Job) {
        let message = format!(
            "⌛️ Deploy *#{}* for project *{}* triggered by {}...",
            job.id, job.project, job.trigger
        );

        let result = self.api.send_message(&telegram::SendMessageParams {
            chat_id: &self.chat_id,
            text: &message,
            parse_mode: Some(telegram::ParseMode::Markdown),
            ..Default::default()
        });

        if let Err(err) = result {
            status!("Unable to send telegram message: {}", err);
        }
    }

    fn after_job(&self, job: &Job, result: &JobResult) {
        let project_name = &job.project;

        let message = match *result {
            Ok(_) => format!(
                "☀️ Deploy for project *{}* completed successfully.",
                project_name
            ),
            Err(ref err) => format!(
                "💔 Deploy for project *{}* failed.\n```\n{}\n```",
                project_name, err
            ),
        };

        let result = self.api.send_message(&telegram::SendMessageParams {
            chat_id: &self.chat_id,
            text: &message,
            parse_mode: Some(telegram::ParseMode::Markdown),
            ..Default::default()
        });

        if let Err(err) = result {
            status!("Unable to send telegram message: {}", err);
        }
    }
}
