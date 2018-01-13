mod model;

pub use self::model::*;

use super::config::get_projects;
use super::ipc::{Receiver, Server};
use std::process;

pub fn start_worker() {
    let projects = match get_projects() {
        Ok(projects) => projects,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };

    let server = Server::new().unwrap();
    let receiver: Receiver<Job> = server.connect().unwrap();

    for job in receiver {
        println!("{:?}", job);
    }
}
