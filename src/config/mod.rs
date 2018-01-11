mod model;
mod find;

pub use self::model::*;

use self::find::find_config_files;
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use std::io::{self, Read};
use std::fmt;
use toml;

#[derive(Debug)]
pub enum ProjectError {
    ListError,
    ReadError(PathBuf),
    ParseError(PathBuf, toml::de::Error),
}

impl fmt::Display for ProjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ProjectError::ListError => write!(f, "unable to list project config files"),
            ProjectError::ReadError(ref path) => {
                write!(f, "unable to read config file {}", path.to_string_lossy())
            }
            ProjectError::ParseError(ref path, ref err) => write!(
                f,
                "error parsing config file {}\n{}:",
                path.to_string_lossy(),
                err
            ),
        }
    }
}

fn read_project_file(path: &PathBuf) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn get_projects() -> Result<HashMap<String, Project>, ProjectError> {
    let config_files = match find_config_files() {
        Ok(files) => files,
        Err(..) => return Err(ProjectError::ListError),
    };

    let mut projects = HashMap::new();

    for config_file in config_files {
        let mut string = match read_project_file(&config_file) {
            Ok(string) => string,
            Err(..) => return Err(ProjectError::ReadError(config_file)),
        };

        let project = match toml::from_str(&string) {
            Ok(project) => project,
            Err(err) => return Err(ProjectError::ParseError(config_file, err)),
        };

        // TODO: define behaviour around invalid file names
        let project_name = config_file
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        projects.insert(project_name, project);
    }

    Ok(projects)
}
