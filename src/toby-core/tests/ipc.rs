mod context;

use self::context::TempContext;
use futures::{Future, Stream};
use std::sync::mpsc::sync_channel;
use std::sync::Arc;
use std::thread;
use toby_core::ipc::{IpcClient, IpcMessage, IpcServerBuilder};
use toby_core::job::{Job, Trigger};
use tokio;

#[test]
fn test_ipc() {
    let (tx, rx) = sync_channel(128);
    let (ready_tx, ready_rx) = sync_channel(128);

    let context = Arc::new(TempContext::create().unwrap());

    {
        let context = context.clone();

        thread::spawn(move || {
            tokio::run(
                IpcServerBuilder::new(&context)
                    .bind()
                    .map_err(|err| panic!("Error: {}", err))
                    .inspect(move |_| {
                        ready_tx.send(()).unwrap();
                    }).and_then(|server| {
                        server
                            .incoming()
                            .map_err(|err| panic!("Error: {}", err))
                            .for_each(move |message| {
                                tx.send(message).unwrap();

                                Ok(())
                            })
                    }),
            );
        });
    }

    {
        let context = context.clone();

        thread::spawn(move || {
            ready_rx.recv().unwrap();

            tokio::run(
                IpcClient::connect(&context)
                    .map_err(|err| panic!("Error: {}", err))
                    .and_then(|client| {
                        let send_future = client.send(&IpcMessage::Job(Job {
                            trigger: Trigger::Cli,
                            project: "foo".into(),
                        }));

                        send_future
                            .map_err(|err| panic!("Error: {}", err))
                            .map(|_| ())
                    }),
            );
        });
    }

    let message = rx.recv().unwrap();

    assert_eq!(
        IpcMessage::Job(Job {
            trigger: Trigger::Cli,
            project: "foo".into(),
        }),
        message
    );
}
