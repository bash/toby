#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]

use toby_core::ipc::{IpcClient, IpcMessage};
use toby_core::job::JobTrigger;
use toby_core::Context;

fn main() {
    let context = Context::default_context();
    let mut client = IpcClient::connect(context).unwrap();

    client
        .send(&IpcMessage::Job {
            project: "foo".into(),
            trigger: JobTrigger::Cli,
        }).unwrap();
}
