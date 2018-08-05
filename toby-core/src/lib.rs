#![feature(crate_in_paths)]

extern crate bincode;
#[macro_use]
extern crate lazy_static;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub const VERSION: &str = env!("TOBY_VERSION");

mod context;

pub mod ipc;
pub mod job;

pub use context::Context;
