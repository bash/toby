use crate::config::get_config;
use crate::server::start_server;
use crate::unwrap_err;
use crate::worker::start_worker;
use std::sync::mpsc::sync_channel;
use std::thread;

// TODO: what value should I have here?
// Note to future self: 8 was picked arbitrarily
const CHANNEL_BOUND: usize = 8;

pub fn start() {
    let config = unwrap_err!(get_config());

    let (sender, receiver) = sync_channel(CHANNEL_BOUND);

    {
        let config = config.clone();

        thread::spawn(move || {
            start_worker(&config, &receiver);
        });
    }

    start_server(config, sender);
}
