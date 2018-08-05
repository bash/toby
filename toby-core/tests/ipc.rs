extern crate toby_core;

use std::sync::mpsc::channel;
use std::thread;
use toby_core::ipc::{IpcClient, IpcMessage, IpcServer};
use toby_core::model::job::JobTrigger;

#[test]
fn test_ipc() {
    let (tx, rx) = channel();
    let (ready_tx, ready_rx) = channel();

    thread::spawn(move || {
        let mut server = IpcServer::bind().unwrap();

        ready_tx.send(()).unwrap();

        let message = server.receive().unwrap();

        tx.send(message).unwrap();
    });

    thread::spawn(move || {
        ready_rx.recv().unwrap();

        let mut client = IpcClient::connect().unwrap();

        client
            .send(&IpcMessage::Job {
                trigger: JobTrigger::Cli,
                project: "foo".into(),
            }).unwrap();
    });

    let message = rx.recv().unwrap();

    assert_eq!(
        IpcMessage::Job {
            trigger: JobTrigger::Cli,
            project: "foo".into(),
        },
        message
    );
}
