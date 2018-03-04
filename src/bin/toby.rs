#![feature(use_extern_macros)]

#[macro_use]
extern crate clap;
extern crate nanoid;
extern crate toby;

use clap::{AppSettings, SubCommand};
use std::thread;
use std::time::Duration;
use toby::config::get_config;
use toby::fs::write_telegram_chat_id;
use toby::telegram::{Api, ParseMode, SendMessageParams};
use toby::unwrap_err;

fn print_random_secret() {
    println!("{}", nanoid::simple());
}

fn telegram_setup() {
    let token = nanoid::generate(6);
    let config = unwrap_err!(get_config());
    let api = Api::from_config(&config).expect("Telegram bot token must be configured");
    let sleep_duration = Duration::from_secs(3);

    println!("Send the following message to your bot:\n  /auth {}", token);

    println!("Polling for incoming message...");

    'poll: loop {
        let updates = api.poll_updates().expect("Unable to fetch updates");

        for update in updates {
            if let Some(message) = update.message {
                if let Some((command, params)) = message.bot_command() {
                    match command {
                        // Ignore /start command, because it's required
                        // to enable chatting with the bot
                        "start" => {}
                        "auth" if params == token => {
                            let chat_id = message.chat.id;

                            println!("Message received. Saving chat id ({})...", chat_id);

                            write_telegram_chat_id(chat_id)
                                .expect("Unable to save telegram chat id");

                            api.send_message(&SendMessageParams {
                                text: "🎉 Congratulations! Toby is now set up and will send notifications to this chat.",
                                chat_id: &chat_id.to_string(),
                                parse_mode: Some(ParseMode::Markdown),
                                ..Default::default()
                            }).expect("Unable to send message");

                            break 'poll;
                        }
                        _ => {
                            println!(
                                "Invalid command: /{} {}. Did you mistype the token?",
                                command, params
                            );
                        }
                    };
                }
            }
        }

        thread::sleep(sleep_duration);
    }
}

fn main() {
    let matches = toby::clap_app!()
        .subcommand(SubCommand::with_name("gen-secret").about("Generates a new, random secret"))
        .subcommand(SubCommand::with_name("telegram-setup").about("Sets up the telegram bot"))
        .settings(&[AppSettings::SubcommandRequired])
        .get_matches();

    match matches.subcommand_name().unwrap() {
        "gen-secret" => print_random_secret(),
        "telegram-setup" => telegram_setup(),
        _ => unreachable!(),
    }
}
