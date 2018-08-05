extern crate toby_core;

use toby_core::ipc::IpcServer;
use toby_core::Context;

fn main() {
    let context = Context::default_context();
    let mut server = IpcServer::bind(context).unwrap();

    loop {
        let msg = server.receive().unwrap();

        println!("Received message: {:?}", msg);
    }
}
