use super::JobResult;
use super::model::Job;
use crate::config::{Config, SendLog};
use crate::fs::job_log_path;
use crate::status;
use crate::telegram;
use reqwest;

#[derive(Debug)]
struct TelegramHook {
    api: telegram::Api,
    chat_id: String,
    send_log: SendLog,
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
    pub(crate) fn from_config(config: &Config, telegram_chat_id: Option<i64>) -> Self {
        let telegram = TelegramHook::from_config(config, telegram_chat_id);

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
    fn from_config(config: &Config, chat_id: Option<i64>) -> Option<Self> {
        let telegram = config.main.telegram.as_ref();

        telegram
            .and_then(|telegram| chat_id.map(|chat_id| (telegram, chat_id)))
            .map(|(telegram, chat_id)| TelegramHook {
                api: telegram::Api::new(reqwest::Client::new(), &telegram.token),
                chat_id: chat_id.to_string(),
                send_log: telegram.send_log,
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

        let send_result = self.api.send_message(&telegram::SendMessageParams {
            chat_id: &self.chat_id,
            text: &message,
            parse_mode: Some(telegram::ParseMode::Markdown),
            ..Default::default()
        });

        if let Err(err) = send_result {
            status!("Unable to send telegram message: {}", err);
        }

        if self.send_log.should_send(result.is_ok()) {
            let path = job_log_path(project_name, job.id);

            let send_result = self.api.send_document(telegram::SendDocumentParams {
                chat_id: self.chat_id.clone(),
                document: telegram::File::InputFile(path.to_string_lossy().into()),
            });

            if let Err(err) = send_result {
                status!("Unable to send telegram message: {}", err);
            }
        }
    }
}
