#![feature(use_extern_macros)]

#[macro_use]
extern crate clap;
extern crate nanoid;
extern crate toby;

use clap::{AppSettings, SubCommand};
use toby::cli::toby::{gen_secret, telegram_setup};

fn main() {
    let matches = toby::clap_app!()
        .subcommand(SubCommand::with_name("gen-secret").about("Generates a new, random secret"))
        .subcommand(SubCommand::with_name("telegram-setup").about("Sets up the telegram bot"))
        .settings(&[AppSettings::SubcommandRequired])
        .get_matches();

    match matches.subcommand_name().unwrap() {
        "gen-secret" => gen_secret(),
        "telegram-setup" => telegram_setup(),
        _ => unreachable!(),
    }
}
