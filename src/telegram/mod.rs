mod model;

pub use self::model::*;
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

    pub fn send_message(&self, params: &SendMessageParams) -> reqwest::Result<Response<Message>> {
        self.client
            .post(&format!("{}/sendMessage", self.api_url))
            .form(&params)
            .send()?
            .json()
    }
}
