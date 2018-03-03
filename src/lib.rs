#![feature(plugin, decl_macro, option_filter, slice_concat_ext, custom_derive, use_extern_macros,
           inclusive_range_syntax, crate_in_paths)]
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

pub mod config;
pub mod worker;
pub mod server;
pub mod telegram;
pub mod database;
pub mod fs;

pub macro clap_app() {
    {
        let version = env!("TOBY_VERSION");

        app_from_crate!()
            .version(version)
            .about("🤖 Toby the friendly server bot")
    }
}

pub macro status {
    ($fmt:expr) => {
        println!($fmt);
    },
    ($fmt:expr, $($arg:tt)*) => {
        println!($fmt, $($arg)*);
    }
}
