use super::JobResult;
use super::model::Job;
use crate::config::Config;
use crate::slack;
use crate::status;
use crate::telegram;
use reqwest;
use std::fmt::Debug;

#[derive(Debug)]
struct TelegramHook {
    api: telegram::Api,
    chat_id: String,
}

#[derive(Debug)]
struct SlackHook {
    api: slack::Api,
    channel: String,
}

#[derive(Debug)]
pub(crate) struct Hooks {
    hooks: Vec<Box<Hook>>,
}

pub(crate) trait Hook: Debug {
    fn before_job(&self, job: &Job);
    fn after_job(&self, job: &Job, result: &JobResult);
}

impl Hooks {
    pub(crate) fn from_config(config: &Config, telegram_chat_id: Option<i64>) -> Self {
        let mut hooks: Vec<Box<Hook>> = Vec::new();

        if let Some(telegram) = TelegramHook::from_config(config, telegram_chat_id) {
            hooks.push(Box::new(telegram));
        }

        if let Some(slack) = SlackHook::from_config(config) {
            hooks.push(Box::new(slack));
        }

        Hooks { hooks }
    }
}

impl Hook for Hooks {
    fn before_job(&self, job: &Job) {
        for hook in &self.hooks {
            hook.before_job(job);
        }
    }

    fn after_job(&self, job: &Job, result: &JobResult) {
        for hook in &self.hooks {
            hook.after_job(job, result);
        }
    }
}

impl TelegramHook {
    fn from_config(config: &Config, chat_id: Option<i64>) -> Option<Self> {
        let telegram = config.main.telegram.as_ref();

        telegram
            .and_then(|telegram| chat_id.map(|chat_id| (telegram, chat_id)))
            .map(|(telegram, chat_id)| TelegramHook {
                api: telegram::Api::new(reqwest::Client::new(), &telegram.token),
                chat_id: chat_id.to_string(),
            })
    }
}

impl SlackHook {
    fn from_config(config: &Config) -> Option<Self> {
        let slack = config.main.slack.as_ref();

        slack.map(|slack| SlackHook {
            api: slack::Api::new(reqwest::Client::new(), slack.bot_token.clone()),
            channel: slack.channel.clone(),
        })
    }
}

impl Hook for TelegramHook {
    fn before_job(&self, job: &Job) {
        let message = format!(
            "⌛️ Job *#{}* for project *{}* triggered by {}...",
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
                "☀️ Job for project *{}* completed successfully.",
                project_name
            ),
            Err(ref err) => format!(
                "💔 Job for project *{}* failed.\n```\n{}\n```",
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

impl Hook for SlackHook {
    fn before_job(&self, job: &Job) {
        let message = format!(
            "⌛️ Job *#{}* for project *{}* triggered by {}...",
            job.id, job.project, job.trigger
        );

        let result = self.api.post_message(&slack::PostMessageParams {
            channel: &self.channel,
            text: &message,
        });

        if let Err(err) = result {
            status!("Unable to send slack message: {}", err);
        }
    }

    fn after_job(&self, job: &Job, result: &JobResult) {
        let project_name = &job.project;

        let message = match *result {
            Ok(_) => format!(
                "☀️ Job for project *{}* completed successfully.",
                project_name
            ),
            Err(ref err) => format!(
                "💔 Job for project *{}* failed.\n```\n{}\n```",
                project_name, err
            ),
        };

        let result = self.api.post_message(&slack::PostMessageParams {
            channel: &self.channel,
            text: &message,
        });

        if let Err(err) = result {
            status!("Unable to send slack message: {}", err);
        }
    }
}
