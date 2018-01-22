use clap::{AppSettings, ArgMatches, SubCommand};

pub fn get_matches<'a>() -> ArgMatches<'a> {
    app_from_crate!()
        .subcommand(SubCommand::with_name("server").about("Starts the http server"))
        .subcommand(SubCommand::with_name("worker").about("Starts the worker"))
        .subcommand(SubCommand::with_name("gen-secret").about("Generates a new, random secret"))
        .settings(&[AppSettings::SubcommandRequired])
        .get_matches()
}
