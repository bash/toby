use crate::config::Config;
use reqwest;
use reqwest::header::{Authorization, Bearer};
use serde::Serialize;
use std::error;
use std::fmt;
use std::result;

pub type Result = result::Result<(), Error>;

#[derive(Debug)]
pub enum Error {
    RequestError(reqwest::Error),
    SlackError(String),
}

#[derive(Deserialize)]
pub struct Response {
    ok: bool,
    error: Option<String>,
}

#[derive(Serialize, Debug, Default)]
pub struct PostMessageParams<'a, 'b> {
    pub channel: &'a str,
    pub text: &'b str,
}

#[derive(Debug)]
pub struct Api {
    client: reqwest::Client,
    token: String,
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
            Error::SlackError(..) => "the slack api returned an error",
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
            Error::SlackError(ref description) => write!(f, "slack error: {}", description),
        }
    }
}

impl Api {
    pub fn new<S: Into<String>>(client: reqwest::Client, token: S) -> Self {
        Api {
            client,
            token: token.into(),
        }
    }

    pub fn from_config(config: &Config) -> Option<Self> {
        let slack = config.main.slack.as_ref();

        slack.map(|slack| Api::new(reqwest::Client::new(), slack.bot_token.clone()))
    }

    fn call_method<T: Serialize + ?Sized>(&self, name: &str, params: &T) -> Result {
        let resp: Response = self.client
            .post(&format!("https://slack.com/api/{}", name))
            .header(Authorization(Bearer {
                token: self.token.clone(),
            }))
            .form(&params)
            .send()?
            .json()?;

        if !resp.ok {
            let error = resp.error.expect("error should be set when ok = false");

            return Err(Error::SlackError(error));
        }

        Ok(())
    }

    pub fn post_message(&self, params: &PostMessageParams) -> Result {
        self.call_method("chat.postMessage", params)
    }
}
