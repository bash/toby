#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]

use toby_core::ipc::IpcServerBuilder;
use toby_core::Context;

fn main() {
    let context = Context::default_context();
    let mut server = IpcServerBuilder::new(context).bind().unwrap();

    loop {
        let msg = server.receive().unwrap();

        println!("Received message: {:?}", msg);
    }
}
