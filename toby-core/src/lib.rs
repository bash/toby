#![feature(crate_in_paths)]

extern crate bincode;
extern crate serde;

#[macro_use]
extern crate serde_derive;

pub mod constants;
pub mod ipc;
pub mod model;
