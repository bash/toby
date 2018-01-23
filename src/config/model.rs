use std::collections::{HashMap, HashSet};

pub type Projects = HashMap<String, Project>;
pub type Tokens = HashMap<String, Token>;

#[derive(Debug, Clone)]
pub struct Config {
    main: MainConfig,
    tokens: Tokens,
    projects: Projects,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    repository: Option<String>,
    scripts: Vec<Script>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Script {
    command: Vec<String>,
    #[serde(default)] allow_failure: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MainConfig {
    listen: ListenConfig,
    telegram: Option<TelegramConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListenConfig {
    port: u16,
    address: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TelegramConfig {
    token: String,
    chat_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Token {
    secret: String,
    access: HashSet<String>,
}

impl Config {
    pub fn new(main: MainConfig, tokens: Tokens, projects: Projects) -> Self {
        Config {
            main,
            tokens,
            projects,
        }
    }

    pub fn main(&self) -> &MainConfig {
        &self.main
    }

    pub fn tokens(&self) -> &Tokens {
        &self.tokens
    }

    pub fn projects(&self) -> &Projects {
        &self.projects
    }
}

impl Project {
    pub fn scripts(&self) -> &[Script] {
        &self.scripts
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

impl MainConfig {
    pub fn listen(&self) -> &ListenConfig {
        &self.listen
    }

    pub fn telegram(&self) -> &Option<TelegramConfig> {
        &self.telegram
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

impl TelegramConfig {
    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn chat_id(&self) -> &str {
        &self.chat_id
    }
}

impl Token {
    pub fn secret(&self) -> &str {
        &self.secret
    }

    pub fn can_access(&self, project: &str) -> bool {
        self.access.contains(project)
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
