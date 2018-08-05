extern crate toby_core;

use toby_core::ipc::IpcServer;

fn main() {
    let mut server = IpcServer::bind().unwrap();

    loop {
        let msg = server.receive().unwrap();

        println!("Received message: {:?}", msg);
    }
}
