mod context;

use self::context::TempContext;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;
use toby_core::ipc::{IpcClient, IpcMessage, IpcServerBuilder};
use toby_core::job::JobTrigger;

#[test]
fn test_ipc() {
    let (tx, rx) = channel();
    let (ready_tx, ready_rx) = channel();

    let context = Arc::new(TempContext::create().unwrap());

    {
        let context = context.clone();

        thread::spawn(move || {
            let mut server = IpcServerBuilder::new(&context).bind().unwrap();

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
