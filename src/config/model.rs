use std::collections::{HashMap, HashSet};

pub(crate) type Projects = HashMap<String, Project>;
pub(crate) type Tokens = HashMap<String, Token>;

#[derive(Debug, Clone)]
pub(crate) struct Config {
    pub(crate) main: MainConfig,
    pub(crate) tokens: Tokens,
    pub(crate) projects: Projects,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub(crate) struct Project {
    pub(crate) repository: Option<String>,
    pub(crate) scripts: Vec<Script>,
    #[serde(default)]
    pub(crate) environment: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub(crate) struct Script {
    pub(crate) command: Vec<String>,
    #[serde(default)]
    pub(crate) allow_failure: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub(crate) struct MainConfig {
    #[serde(default)]
    pub(crate) listen: ListenConfig,
    pub(crate) telegram: Option<TelegramConfig>,
    pub(crate) tls: Option<TlsConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub(crate) struct ListenConfig {
    #[serde(default = "default_port")]
    pub(crate) port: u16,
    #[serde(default = "default_address")]
    pub(crate) address: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub(crate) struct TelegramConfig {
    pub(crate) token: String,
    #[serde(default)]
    pub(crate) send_log: SendLog,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub(crate) enum SendLog {
    Never,
    Always,
    Success,
    Failure,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub(crate) struct TlsConfig {
    certificate: String,
    certificate_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub(crate) struct Token {
    pub(crate) secret: String,
    pub(crate) access: HashSet<String>,
}

fn default_port() -> u16 {
    8629
}

fn default_address() -> String {
    "0.0.0.0".into()
}

impl Config {
    pub(crate) fn new(main: MainConfig, tokens: Tokens, projects: Projects) -> Self {
        Config {
            main,
            tokens,
            projects,
        }
    }
}

impl TlsConfig {
    pub(crate) fn certificate(&self) -> &str {
        &self.certificate
    }

    pub(crate) fn certificate_key(&self) -> &str {
        &self.certificate_key
    }
}

impl Token {
    pub(crate) fn can_access(&self, project: &str) -> bool {
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

impl Default for SendLog {
    fn default() -> Self {
        SendLog::Never
    }
}

impl SendLog {
    pub(crate) fn should_send(self, successful: bool) -> bool {
        match self {
            SendLog::Always => true,
            SendLog::Success if successful => true,
            SendLog::Failure if !successful => true,
            _ => false,
        }
    }
}
