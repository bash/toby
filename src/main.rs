#![feature(plugin, decl_macro, option_filter, slice_concat_ext)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate clap;
extern crate ipc_channel;
#[macro_use]
extern crate lazy_static;
extern crate nanoid;
extern crate reqwest;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate tempdir;
extern crate toml;

mod config;
mod cli;
mod worker;
mod server;
mod ipc;
mod telegram;

use cli::get_matches;
use worker::start_worker;
use server::start_server;

fn print_token() {
    println!("{}", nanoid::simple());
}

fn main() {
    let matches = get_matches();
    let subcommand = matches.subcommand_name().unwrap();

    match subcommand {
        "server" => start_server(),
        "worker" => start_worker(),
        "token" => print_token(),
        _ => unreachable!(),
    }
}
