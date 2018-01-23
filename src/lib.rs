#![feature(plugin, decl_macro, option_filter, slice_concat_ext, custom_derive)]
#![plugin(rocket_codegen)]

extern crate clap;
extern crate ipc_channel;
extern crate nanoid;
extern crate reqwest;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate tempdir;
extern crate toml;

pub mod config;
pub mod worker;
pub mod server;
mod ipc;
pub mod telegram;
