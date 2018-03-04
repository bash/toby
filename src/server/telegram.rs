use crate::config::Config;
use crate::telegram::{Api, SetWebhookParams, Update};
use nanoid;
use rocket::{Rocket, State};
use rocket::fairing::{self, Fairing};
use rocket::http::Status;
use rocket::response::Failure;
use rocket_contrib::Json;

const TELEGRAM_WEBHOOK_TOKEN_LEN: usize = 48;

pub(crate) struct TelegramWebhookToken(String);
pub(crate) struct TelegramFairing;

impl TelegramWebhookToken {
    pub(crate) fn new() -> Self {
        TelegramWebhookToken(nanoid::generate(TELEGRAM_WEBHOOK_TOKEN_LEN))
    }
}

impl Fairing for TelegramFairing {
    fn info(&self) -> fairing::Info {
        fairing::Info {
            name: "Telegram",
            kind: fairing::Kind::Attach,
        }
    }

    fn on_attach(&self, rocket: Rocket) -> Result<Rocket, Rocket> {
        let (token, public_url, api) = {
            let config = rocket
                .state::<Config>()
                .expect("config must be managed by rocket");

            let token = TelegramWebhookToken::new();

            let public_url = format!(
                "https://{}:{}/hooks/telegram/{}",
                config.main.domain_name, config.main.listen.port, token.0
            );

            let api = Api::from_config(config);

            (token, public_url, api)
        };

        let api = match api {
            Some(val) => val,
            None => return Ok(rocket),
        };

        api.set_webhook(&SetWebhookParams {
            url: &public_url,
            ..Default::default()
        }).expect("Unable to register webhook with telegram");

        println!("Public URL: {}", public_url);

        Ok(rocket.manage(token))
    }
}

#[post("/hooks/telegram/<token>", format = "application/json", data = "<update>")]
pub(crate) fn telegram_hook(
    webhook_token: State<TelegramWebhookToken>,
    token: String,
    update: Json<Update>,
) -> Result<(), Failure> {
    if webhook_token.0 != token {
        return Err(Failure(Status::Forbidden));
    }

    println!("{:#?}", update);
    println!(
        "{:#?}",
        update.message.as_ref().and_then(|msg| msg.bot_command())
    );

    Ok(())
}
