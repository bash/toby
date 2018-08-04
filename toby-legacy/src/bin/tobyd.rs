#![feature(use_extern_macros)]

#[macro_use]
extern crate clap;
extern crate toby;

use toby::clap_app;
use toby::cli::tobyd::start;

fn main() {
    let _ = clap_app!().get_matches();

    start();
}
