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
    source: &'a (dyn ConfigSource),
}

///
/// Loads configuration from the filesystem.
///
#[derive(Debug)]
pub struct FsConfigSource<'a> {
    context: &'a Context,
}

pub trait ConfigSource: fmt::Debug {
    fn load_main_config(&self) -> Result<MainConfig, ConfigError>;
    fn load_tokens(&self) -> Result<Tokens, ConfigError>;
    fn load_projects(&self) -> Result<Projects, ConfigError>;
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

impl<'a> ConfigLoader<'a> {
    pub fn new(source: &'a dyn ConfigSource) -> Self {
        Self { source }
    }

    pub fn load(&self) -> Result<Config, ConfigError> {
        let main = self.source.load_main_config()?;
        let tokens = self.source.load_tokens()?;
        let projects = self.source.load_projects()?;

        Ok(Config {
            main,
            tokens,
            projects,
        })
    }
}

impl<'a> FsConfigSource<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self { context }
    }
}

impl<'a> ConfigSource for FsConfigSource<'a> {
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

#[cfg(test)]
mod test {
    use super::super::{Config, MainConfig, Project, Token};
    use super::*;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_config_loader() {
        #[derive(Debug)]
        struct MockConfigSource;

        impl ConfigSource for MockConfigSource {
            fn load_main_config(&self) -> Result<MainConfig, ConfigError> {
                Ok(Default::default())
            }

            fn load_tokens(&self) -> Result<Tokens, ConfigError> {
                let mut tokens = HashMap::new();
                tokens.insert(
                    "bot".into(),
                    Token {
                        secret: "much_secret_wow".into(),
                        access: HashSet::new(),
                    },
                );
                Ok(Tokens(tokens))
            }

            fn load_projects(&self) -> Result<Projects, ConfigError> {
                let mut projects = HashMap::new();
                projects.insert(
                    "dreams".into(),
                    Project {
                        scripts: vec![],
                        environment: HashMap::new(),
                    },
                );
                Ok(Projects(projects))
            }
        }

        let main: MainConfig = Default::default();

        let mut projects = HashMap::new();
        projects.insert(
            "dreams".into(),
            Project {
                scripts: vec![],
                environment: HashMap::new(),
            },
        );

        let mut tokens = HashMap::new();
        tokens.insert(
            "bot".into(),
            Token {
                secret: "much_secret_wow".into(),
                access: HashSet::new(),
            },
        );

        let config = ConfigLoader::new(&MockConfigSource).load().unwrap();

        assert_eq!(
            Config {
                main,
                projects: Projects(projects),
                tokens: Tokens(tokens),
            },
            config
        );
    }
}
