#![warn(rust_2018_idioms)]

use futures::{future, Future, Stream};
use toby_core::cancelation::{cancelation_token, CancelableStreamExt};
#[cfg(feature = "enable-user-switch")]
use toby_core::config::ConfigLoader;
#[cfg(feature = "enable-user-switch")]
use toby_core::identity::Identity;
use toby_core::ipc::IpcServerBuilder;
use toby_core::Context;
use tokio;

mod plugin;

fn main() {
    let context = Context::default_context();
    let registry = plugin::load_plugins(&["toby_telegram"]);

    println!("Registered hooks: {}", registry.job_hooks.len());

    #[cfg(feature = "enable-user-switch")]
    let config = ConfigLoader::new(&context).load().unwrap();

    #[cfg(feature = "enable-user-switch")]
    let identity = Identity::load(config.user(), config.group()).unwrap();

    let server_builder = IpcServerBuilder::new(context);

    #[cfg(feature = "enable-user-switch")]
    let server_builder = server_builder.owner(&identity);

    let (cancelation_token, _) = cancelation_token();

    tokio::run(
        server_builder
            .bind()
            .map_err(|err| println!("Error starting ipc server: {}", err))
            .and_then(|server| {
                server
                    .incoming()
                    .cancelable(cancelation_token)
                    .map_err(|err| println!("Error receiving message: {}", err))
                    .for_each(|msg| {
                        println!("Received message: {:?}", msg);

                        future::ok(())
                    })
            }),
    );
}
