extern crate toby_core;

use toby_core::ipc::IpcClient;

fn main() {
    let mut client: IpcClient<String> = IpcClient::connect().unwrap();

    client.send(&("foo bar baz".into())).unwrap();
}
