mod model;

use reqwest;
pub use self::model::*;

pub fn send_message(
    client: &reqwest::Client,
    token: &str,
    params: &SendMessageParams,
) -> reqwest::Result<reqwest::Response> {
    client
        .post(&format!(
            "https://api.telegram.org/bot{}/sendMessage",
            token
        ))
        .form(&params)
        .send()
}
