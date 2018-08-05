#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

pub const VERSION: &str = env!("TOBY_VERSION");

mod context;

pub mod ipc;
pub mod job;

pub use self::context::Context;
