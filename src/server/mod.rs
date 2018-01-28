use super::worker::Job;
use super::config::Config;
use rocket;
use rocket::fairing::AdHoc;
use rocket::config::{ConfigBuilder, Environment};
use std::sync::mpsc::SyncSender;
use super::status;

mod telegram;
mod deploy;

pub fn start_server(config: Config, sender: SyncSender<Job>) {
    let rocket_config = {
        let builder = ConfigBuilder::new(Environment::Production)
            .address(config.main().listen().address().clone())
            .port(config.main().listen().port());

        if let Some(ref tls) = *config.main().tls() {
            builder.tls(tls.certificate(), tls.certificate_key())
        } else {
            builder
        }
    }.unwrap();

    rocket::custom(rocket_config, true)
        .attach(AdHoc::on_launch(|_| status!("Server is starting...")))
        .manage(sender)
        .manage(config)
        .mount("/", routes![deploy::deploy, telegram::telegram_hook])
        .launch();
}
