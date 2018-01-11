mod model;

pub use self::model::*;

use super::config::get_projects;
use std::process;
use ipc_channel::ipc::IpcOneShotServer;
use std::fs::{self, File};
use std::io::Write;

const IPC_NAME_FILE: &'static str = "/tmp/toby-ipc-server-name";

pub fn start_worker() {
    let projects = match get_projects() {
        Ok(projects) => projects,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };

    let (server, name) = IpcOneShotServer::<Job>::new().unwrap();

    {
        let mut ipc_name_file =
            File::create(IPC_NAME_FILE).expect("unable to create /tmp/toby-ipc-server-name");

        ipc_name_file
            .write_all(name.as_bytes())
            .expect("unable to write to /tmp/toby-ipc-server-name");
    }

    let (receiver, ..) = server.accept().expect("unable to connect with ipc client");

    fs::remove_file(IPC_NAME_FILE).expect("unable to delete /tmp/toby-ipc-server-name");

    loop {
        let job = receiver.recv().expect("unable to receive job");

        println!("{:?}", job);
    }

    // TODO: implement drop for ipc_name_file
}
