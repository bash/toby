#[macro_use]
extern crate clap;
extern crate ipc_channel;
#[macro_use]
extern crate lazy_static;
extern crate nanoid;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod config;
mod cli;
mod worker;

use cli::get_matches;
use worker::start_worker;

fn print_token() {
    println!("{}", nanoid::simple());
}

fn main() {
    let matches = get_matches();
    let subcommand = matches.subcommand_name().unwrap();

    match subcommand {
        "server" => unimplemented!(),
        "worker" => start_worker(),
        "token" => print_token(),
        _ => unreachable!(),
    }
}
