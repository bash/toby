extern crate toby;

use toby::config::get_config;
use toby::server::start_server;
use toby::worker::start_worker;
use std::sync::mpsc::sync_channel;
use std::thread;

// TODO: what value should I have here?
// Note to future self: 8 was picked arbitrarily
const CHANNEL_BOUND: usize = 8;

fn main() {
    let config = get_config().expect("unable to read config");
    let (sender, receiver) = sync_channel(CHANNEL_BOUND);

    {
        let config = config.clone();

        thread::spawn(move || {
            start_worker(config, receiver);
        });
    }

    start_server(config, sender);
}
