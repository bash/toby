extern crate tempdir;
extern crate toby_core;

use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;
use tempdir::TempDir;
use toby_core::ipc::{IpcClient, IpcMessage, IpcServer};
use toby_core::job::JobTrigger;
use toby_core::Context;

#[test]
fn test_ipc() {
    let (tx, rx) = channel();
    let (ready_tx, ready_rx) = channel();

    let config_dir = TempDir::new("toby-ipc").unwrap();
    let log_dir = TempDir::new("toby-ipc").unwrap();
    let runtime_dir = TempDir::new("toby-ipc").unwrap();

    let context = Arc::new(Context::new(
        config_dir.path().to_owned(),
        log_dir.path().to_owned(),
        runtime_dir.path().to_owned(),
    ));

    {
        let context = context.clone();

        thread::spawn(move || {
            let mut server = IpcServer::bind(&context).unwrap();

            ready_tx.send(()).unwrap();

            let message = server.receive().unwrap();

            tx.send(message).unwrap();
        });
    }

    {
        let context = context.clone();

        thread::spawn(move || {
            ready_rx.recv().unwrap();

            let mut client = IpcClient::connect(&context).unwrap();

            client
                .send(&IpcMessage::Job {
                    trigger: JobTrigger::Cli,
                    project: "foo".into(),
                }).unwrap();
        });
    }

    let message = rx.recv().unwrap();

    assert_eq!(
        IpcMessage::Job {
            trigger: JobTrigger::Cli,
            project: "foo".into(),
        },
        message
    );
}
