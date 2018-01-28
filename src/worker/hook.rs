use super::error::DeployResult;
use super::model::Job;
use crate::telegram;
use crate::config::Config;
use crate::status;
use reqwest;

pub struct TelegramHook {
    api: telegram::Api,
    chat_id: String,
}

pub trait Hook {
    fn before_deploy(&self, job: &Job);
    fn after_deploy(&self, job: &Job, result: &DeployResult);
}

impl TelegramHook {
    pub fn from_config(config: &Config) -> Option<Self> {
        config.main().telegram().map(|ref telegram| TelegramHook {
            api: telegram::Api::new(reqwest::Client::new(), telegram.token()),
            chat_id: telegram.chat_id().to_string(),
        })
    }
}

impl Hook for TelegramHook {
    fn before_deploy(&self, job: &Job) {
        let message = format!(
            "⌛️ Deploy for project *{}* triggered by {}...",
            job.project(),
            job.trigger()
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

    fn after_deploy(&self, job: &Job, result: &DeployResult) {
        let project_name = job.project();

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
