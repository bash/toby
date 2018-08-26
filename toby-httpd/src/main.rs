#![warn(rust_2018_idioms)]

use toby_core::config::{ConfigLoader, FsConfigSource};
#[cfg(feature = "enable-user-switch")]
use toby_core::identity::{set_current, Identity};
use toby_core::ipc::IpcClient;
use toby_core::Context;

fn main() {
    let context = Context::default_context();

    let config = ConfigLoader::new(&FsConfigSource::new(context))
        .load()
        .unwrap();

    #[cfg(feature = "enable-user-switch")]
    {
        let identity = Identity::load(config.user(), config.group()).unwrap();
        set_current(&identity).unwrap();
    }

    let mut client = IpcClient::connect(context).unwrap();
}
