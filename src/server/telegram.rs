use crate::config::Config;
use crate::telegram::Update;
use rocket::State;
use rocket_contrib::Json;

#[post("/hooks/telegram/<token>", format = "application/json", data = "<update>")]
pub fn telegram_hook(config: State<Config>, token: String, update: Json<Update>) {
    println!("{:?} {:?}", config, token);
    println!("{:#?}", update);
    println!(
        "{:#?}",
        update.message.as_ref().and_then(|msg| msg.bot_command())
    );
}
