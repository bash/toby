mod model;

pub use self::model::*;
use crate::config::Config;
use reqwest;

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

    pub fn from_config(config: &Config) -> Option<Self> {
        let telegram = config.main.telegram.as_ref();

        telegram.map(|telegram| Api::new(reqwest::Client::new(), &telegram.token))
    }

    pub fn send_message(&self, params: &SendMessageParams) -> reqwest::Result<Response<Message>> {
        self.client
            .post(&format!("{}/sendMessage", self.api_url))
            .form(&params)
            .send()?
            .json()
    }

    pub fn set_webhook(&self, params: &SetWebhookParams) -> reqwest::Result<Response<Message>> {
        self.client
            .post(&format!("{}/setWebhook", self.api_url))
            .form(&params)
            .send()?
            .json()
    }
}
