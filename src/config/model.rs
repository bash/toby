use std::collections::HashMap;

pub type Projects = HashMap<String, Project>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    token: String,
    repository: Option<String>,
    scripts: Vec<Script>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Script {
    command: Vec<String>,
    #[serde(default)] allow_failure: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    listen: ListenConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListenConfig {
    port: u16,
    address: String,
}

impl Project {
    pub fn scripts(&self) -> &[Script] {
        &self.scripts
    }

    pub fn token(&self) -> &str {
        &self.token
    }
}

impl Script {
    pub fn command(&self) -> &[String] {
        &self.command
    }

    pub fn allow_failure(&self) -> bool {
        self.allow_failure
    }
}

impl Config {
    pub fn listen(&self) -> &ListenConfig {
        &self.listen
    }
}

impl ListenConfig {
    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

impl Default for ListenConfig {
    fn default() -> Self {
        Self {
            port: 8629,
            address: "0.0.0.0".into(),
        }
    }
}
