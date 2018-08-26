#![warn(rust_2018_idioms)]

use toby_core::config::{ConfigLoader, FsConfigSource};
use toby_core::identity::Identity;
use toby_core::ipc::IpcServerBuilder;
use toby_core::Context;

fn main() {
    let context = Context::default_context();
    let config = ConfigLoader::new(&FsConfigSource::new(context))
        .load()
        .unwrap();

    let identity = Identity::load(config.user(), config.group()).unwrap();

    let server_builder = IpcServerBuilder::new(context);

    #[cfg(feature = "enable-user-switch")]
    let server_builder = server_builder.owner(&identity);

    let mut server = server_builder.bind().unwrap();

    loop {
        let msg = server.receive().unwrap();

        println!("Received message: {:?}", msg);
    }
}
