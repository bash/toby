extern crate toby_core;

use toby_core::ipc::{IpcClient, IpcMessage};
use toby_core::model::job::JobTrigger;

fn main() {
    let mut client = IpcClient::connect().unwrap();

    client
        .send(&IpcMessage::Job {
            project: "foo".into(),
            trigger: JobTrigger::Cli,
        }).unwrap();
}
