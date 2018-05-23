#![feature(
    plugin, decl_macro, option_filter, slice_concat_ext, custom_derive, use_extern_macros,
    crate_in_paths
)]
#![plugin(rocket_codegen)]
#![deny(dead_code)]

extern crate byteorder;
extern crate clap;
extern crate fs2;
extern crate nanoid;
extern crate reqwest;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate tempdir;
extern crate toml;

pub mod cli;
pub(crate) mod config;
pub(crate) mod fs;
pub(crate) mod server;
pub mod telegram;
pub(crate) mod time;
pub(crate) mod worker;

pub macro clap_app() {{
    let version = env!("TOBY_VERSION");

    app_from_crate!()
        .version(version)
        .about("🤖 Toby the friendly server bot")
}}

pub(crate) macro unwrap_err($val:expr) {
    match $val {
        Ok(val) => val,
        Err(err) => {
            eprintln!("{}", err);
            ::std::process::exit(1);
        }
    };
}

pub(crate) macro status {
    ($fmt:expr) => {
        println!($fmt);
    },
    ($fmt:expr, $($arg:tt)*) => {
        println!($fmt, $($arg)*);
    }
}
