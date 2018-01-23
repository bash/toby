#![feature(use_extern_macros)]

#[macro_use]
extern crate clap;
extern crate nanoid;
extern crate toby;

use clap::{AppSettings, SubCommand};

fn print_random_secret() {
    println!("{}", nanoid::simple());
}

fn main() {
    let matches = toby::clap_app!()
        .subcommand(SubCommand::with_name("gen-secret").about("Generates a new, random secret"))
        .settings(&[AppSettings::SubcommandRequired])
        .get_matches();

    match matches.subcommand_name().unwrap() {
        "gen-secret" => print_random_secret(),
        _ => unreachable!(),
    }
}
