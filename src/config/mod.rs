mod model;
mod find;

pub use self::model::*;

use self::find::{find_config_file, find_project_configs};
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use std::io::{self, Read};
use std::fmt;
use toml;

#[derive(Debug)]
pub enum ConfigError {
    ListError,
    ReadError(PathBuf),
    ParseError(PathBuf, toml::de::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigError::ListError => write!(f, "unable to list project config files"),
            ConfigError::ReadError(ref path) => {
                write!(f, "unable to read config file {}", path.to_string_lossy())
            }
            ConfigError::ParseError(ref path, ref err) => write!(
                f,
                "error parsing config file {}\n{}:",
                path.to_string_lossy(),
                err
            ),
        }
    }
}

fn read_file(path: &PathBuf) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn get_projects() -> Result<Projects, ConfigError> {
    let config_files = match find_project_configs() {
        Ok(files) => files,
        Err(..) => return Err(ConfigError::ListError),
    };

    let mut projects = HashMap::new();

    for config_file in config_files {
        let mut string = match read_file(&config_file) {
            Ok(string) => string,
            Err(..) => return Err(ConfigError::ReadError(config_file)),
        };

        let project = match toml::from_str(&string) {
            Ok(project) => project,
            Err(err) => return Err(ConfigError::ParseError(config_file, err)),
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

pub fn get_config() -> Result<Config, ConfigError> {
    let path = find_config_file();

    path.map(PathBuf::from)
        .map(|path| {
            read_file(&path)
                .map_err(|_| ConfigError::ReadError(path.clone()))
                .and_then(|string| {
                    toml::from_str(&string)
                        .map_err(|err| ConfigError::ParseError(path.clone(), err))
                })
        })
        .unwrap_or(Ok(Config::default()))
}
