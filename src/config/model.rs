use std::collections::{HashMap, HashSet};

pub type Projects = HashMap<String, Project>;
pub type Tokens = HashMap<String, Token>;

#[derive(Debug, Clone)]
pub struct Config {
    pub main: MainConfig,
    pub tokens: Tokens,
    pub projects: Projects,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Project {
    pub repository: Option<String>,
    pub scripts: Vec<Script>,
    #[serde(default)]
    pub environment: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Script {
    pub command: Vec<String>,
    #[serde(default)]
    pub allow_failure: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MainConfig {
    #[serde(default)]
    pub listen: ListenConfig,
    pub telegram: Option<TelegramConfig>,
    pub slack: Option<SlackConfig>,
    pub tls: Option<TlsConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ListenConfig {
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_address")]
    pub address: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct TelegramConfig {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SlackConfig {
    pub bot_token: String,
    pub channel: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct TlsConfig {
    certificate: String,
    certificate_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Token {
    pub secret: String,
    pub access: HashSet<String>,
}

fn default_port() -> u16 {
    8629
}

fn default_address() -> String {
    "0.0.0.0".into()
}

impl Config {
    pub fn new(main: MainConfig, tokens: Tokens, projects: Projects) -> Self {
        Config {
            main,
            tokens,
            projects,
        }
    }
}

impl TlsConfig {
    pub fn certificate(&self) -> &str {
        &self.certificate
    }

    pub fn certificate_key(&self) -> &str {
        &self.certificate_key
    }
}

impl Token {
    pub fn can_access(&self, project: &str) -> bool {
        self.access.contains(project)
    }
}

impl Default for ListenConfig {
    fn default() -> Self {
        ListenConfig {
            address: default_address(),
            port: default_port(),
        }
    }
}
