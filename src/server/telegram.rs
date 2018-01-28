use rocket_contrib::Json;
use crate::telegram::{Api, Update};
use crate::config::Config;
use rocket::State;

#[post("/hooks/telegram/<token>", format = "application/json", data = "<update>")]
pub fn telegram_hook(config: State<Config>, token: String, update: Json<Update>) {
    println!("{:#?}", update);
    println!(
        "{:#?}",
        update.message.as_ref().and_then(|msg| msg.bot_command())
    );
}
