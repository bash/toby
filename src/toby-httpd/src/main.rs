#![warn(rust_2018_idioms)]

use futures::Future;
#[cfg(feature = "enable-user-switch")]
use toby_core::config::{ConfigLoader, FsConfigSource};
#[cfg(feature = "enable-user-switch")]
use toby_core::identity::{set_current, Identity};
use toby_core::ipc::IpcClient;
use toby_core::Context;
use tokio;

fn main() {
    let context = Context::default_context();

    #[cfg(feature = "enable-user-switch")]
    let config = ConfigLoader::new(&FsConfigSource::new(context))
        .load()
        .unwrap();

    #[cfg(feature = "enable-user-switch")]
    {
        let identity = Identity::load(config.user(), config.group()).unwrap();
        set_current(&identity).unwrap();
    }

    tokio::run(
        IpcClient::connect(context)
            .map(|_client| ())
            .map_err(|err| println!("Error: {}", err)),
    );
}
