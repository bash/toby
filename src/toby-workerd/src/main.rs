#![warn(rust_2018_idioms)]

use self::plugin::load_plugins;
use futures::{future, Future, Stream};
use std::process;
use toby_core::cancelation::{cancelation_token, CancelableStreamExt};
use toby_core::config::ConfigLoader;
#[cfg(feature = "enable-user-switch")]
use toby_core::identity::Identity;
use toby_core::ipc::IpcServerBuilder;
use toby_core::Context;
use tokio;

mod path;
mod plugin;

fn main() {
    if let Err(err) = main_inner() {
        eprintln!("{}", err);
        process::exit(1);
    }
}

fn main_inner() -> Result<(), Box<dyn std::error::Error>> {
    let context = Context::default_context();
    let config_loader = ConfigLoader::new(&context);
    let config = config_loader.load()?;
    let _plugins = load_plugins(config.plugins(), &config_loader)?;

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

    Ok(())
}
