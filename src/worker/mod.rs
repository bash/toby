mod model;

pub use self::model::*;

use super::config::{get_projects, Project};
use super::ipc::{Receiver, Server};
use std::process::{self, Command};
use std::error::Error;

macro_rules! status {
    ($fmt:expr, $($arg:tt)*) => {
        println!(concat!("[toby] ", $fmt), $($arg)*)
    };
}

pub fn process_project(project_name: &str, project: &Project) {
    status!("Building project {}", project_name);

    for script in project.scripts() {
        let command = script.command();

        status!("Running {:?}", command);

        let status = Command::new(&command[0]).args(&command[1..]).status();

        let failed = match status {
            Ok(status) => !status.success(),
            Err(err) => {
                status!("Execution failed: {}", err);
                true
            }
        };

        if failed && !script.allow_failure() {
            break;
        }
    }
}

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
        let job = job.unwrap();
        let project_name = job.project();

        match projects.get(project_name) {
            Some(project) => process_project(project_name, project),
            None => status!("Project {} does not exist", project_name),
        }
    }
}
