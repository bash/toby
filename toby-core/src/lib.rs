#![feature(rust_2018_preview, decl_macro, box_syntax)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

pub const VERSION: &str = env!("TOBY_VERSION");

mod context;

pub mod config;
pub mod identity;
pub mod ipc;
pub mod job;

pub use self::context::Context;

pub macro path {
    ($x:expr) => {{
        std::path::PathBuf::from($x)
    }},
    ($($y:expr),+) => {{
        let mut path = std::path::PathBuf::new();
        $(
            path.push($y);
        )*
        path
    }}
}
