use super::{Config, MainConfig, Projects, Tokens};
use crate::{path, Context};
use std::error;
use std::fmt;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};
use toml;

const CONFIG_FILE_NAME: &str = "toby.toml";
const TOKENS_FILE_NAME: &str = "tokens.toml";

#[derive(Debug)]
pub enum ConfigError {
    NotFound(PathBuf),
    ListError(Box<dyn error::Error>),
    ReadError(PathBuf, Box<dyn error::Error>),
    ParseError(PathBuf, toml::de::Error),
}

#[derive(Debug)]
pub struct ConfigLoader<'a> {
    context: &'a Context,
}

impl<'a> ConfigLoader<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self { context }
    }

    pub fn load(&self) -> Result<Config, ConfigError> {
        let main = self.load_main_config()?;
        let tokens = self.load_tokens()?;
        let projects = self.load_projects()?;

        Ok(Config {
            main,
            tokens,
            projects,
        })
    }

    pub fn load_custom<T>(&self, name: &str) -> Result<T, ConfigError>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        read_config_file(path!(self.context.config_path(), &format!("{}.toml", name)))
    }

    fn load_main_config(&self) -> Result<MainConfig, ConfigError> {
        read_config_file(path!(self.context.config_path(), CONFIG_FILE_NAME))
    }

    fn load_tokens(&self) -> Result<Tokens, ConfigError> {
        read_config_file(path!(self.context.config_path(), TOKENS_FILE_NAME))
    }

    fn load_projects(&self) -> Result<Projects, ConfigError> {
        Ok(Projects(std::collections::HashMap::new()))
    }
}

fn read_file(path: &Path) -> io::Result<String> {
    let mut reader = BufReader::new(File::open(path)?);
    let mut contents = String::new();

    reader.read_to_string(&mut contents)?;

    Ok(contents)
}

fn read_config_file<T>(path: PathBuf) -> Result<T, ConfigError>
where
    for<'de> T: serde::Deserialize<'de>,
{
    let contents = match read_file(&path) {
        Ok(contents) => contents,
        Err(err) => return Err(ConfigError::ReadError(path, box err)),
    };

    let config = match toml::from_str(&contents) {
        Ok(config) => config,
        Err(err) => return Err(ConfigError::ParseError(path, err)),
    };

    Ok(config)
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ConfigError::ListError(ref err) => {
                write!(f, "Unable to list project config files:\n{}", err)
            }
            ConfigError::ReadError(ref path, ref err) => write!(
                f,
                "Unable to read config file {}:\n{}",
                path.to_string_lossy(),
                err,
            ),
            ConfigError::ParseError(ref path, ref err) => write!(
                f,
                "Error parsing config file {}:\n{}",
                path.to_string_lossy(),
                err
            ),
            ConfigError::NotFound(ref path) => {
                write!(f, "Config file {} does not exist", path.to_string_lossy())
            }
        }
    }
}
