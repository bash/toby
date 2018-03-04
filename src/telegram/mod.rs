mod model;

pub use self::model::*;
use crate::config::Config;
use reqwest;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::error;
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    RequestError(reqwest::Error),
    TelegramError(i64, String),
    IncompleteTelegramError,
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::RequestError(err)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::RequestError(ref err) => err.description(),
            Error::TelegramError(..) => "the telegram api returned an error",
            Error::IncompleteTelegramError => "the telegram api returned an incomplete error",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::RequestError(ref err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::RequestError(ref err) => write!(f, "request error: {}", err),
            Error::TelegramError(code, ref description) => {
                write!(f, "telegram error ({}): {}", code, description)
            }
            Error::IncompleteTelegramError => {
                write!(f, "the telegram api returned an incomplete error")
            }
        }
    }
}

#[derive(Debug)]
pub struct Api {
    client: reqwest::Client,
    api_url: String,
}

impl Api {
    pub fn new(client: reqwest::Client, token: &str) -> Self {
        Api {
            client,
            api_url: format!("https://api.telegram.org/bot{}", token),
        }
    }

    pub(crate) fn from_config(config: &Config) -> Option<Self> {
        let telegram = config.main.telegram.as_ref();

        telegram.map(|telegram| Api::new(reqwest::Client::new(), &telegram.token))
    }

    fn call_method<T: Serialize + ?Sized, R: DeserializeOwned>(
        &self,
        name: &str,
        params: &T,
    ) -> Result<R> {
        let resp: Response<R> = self.client
            .post(&format!("{}/{}", self.api_url, name))
            .form(&params)
            .send()?
            .json()?;

        if !resp.ok {
            return match (resp.error_code, resp.description) {
                (Some(code), Some(description)) => Err(Error::TelegramError(code, description)),
                _ => Err(Error::IncompleteTelegramError),
            };
        }

        // we can safely unwrap, because a response with
        // ok = true always has a result
        Ok(resp.result.unwrap())
    }

    pub fn send_message(&self, params: &SendMessageParams) -> Result<Message> {
        self.call_method("sendMessage", params)
    }

    pub fn set_webhook(&self, params: &SetWebhookParams) -> Result<Message> {
        self.call_method("setWebhook", params)
    }

    pub fn get_updates(&self, params: &GetUpdatesParams) -> Result<Vec<Update>> {
        self.call_method("getUpdates", params)
    }

    ///
    /// Gets updates and makes sure telegram 'forgets' them.
    ///
    pub fn poll_updates(&self) -> Result<Vec<Update>> {
        let updates = self.get_updates(&Default::default())?;

        // this make sure that the updates are forgotten
        // so the next time we call getUpdates we only receive new updates.
        if let Some(last) = updates.last() {
            self.get_updates(&GetUpdatesParams {
                offset: Some(last.update_id + 1),
                ..Default::default()
            })?;
        }

        Ok(updates)
    }
}
