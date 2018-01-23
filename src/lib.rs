#![feature(plugin, decl_macro, option_filter, slice_concat_ext, custom_derive, use_extern_macros)]
#![plugin(rocket_codegen)]

extern crate clap;
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

pub macro clap_app {
    () => {
        {
            let version = option_env!("VERSION").unwrap_or(crate_version!());

            app_from_crate!().version(version).about("🤖 Toby the friendly server bot")
        }
    };
}

pub macro status {
    ($fmt:expr) => {
        println!(concat!("[toby] ", $fmt));
    },
    ($fmt:expr, $($arg:tt)*) => {
        println!(concat!("[toby] ", $fmt), $($arg)*);
    }
}
