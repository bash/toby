use serde::{self, Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::net::{IpAddr, Ipv4Addr};

const DEFAULT_PORT: u16 = 8629;
const DEFAULT_USER: &str = "toby";
const DEFAULT_GROUP: &str = "toby";

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Tokens(pub(super) HashMap<String, Token>);

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Projects(pub(super) HashMap<String, Project>);

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Config {
    pub(super) main: MainConfig,
    pub(super) tokens: Tokens,
    pub(super) projects: Projects,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Project {
    pub(super) scripts: Vec<Script>,
    #[serde(default)]
    pub(super) environment: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub(super) struct Script {
    pub(super) command: Vec<String>,
    #[serde(default)]
    pub(super) allow_failure: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct MainConfig {
    #[serde(default = "default_group")]
    pub(super) user: String,
    #[serde(default = "default_user")]
    pub(super) group: String,
    #[serde(default)]
    pub(super) listen: ListenConfig,
    pub(super) telegram: Option<TelegramConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub(super) struct ListenConfig {
    #[serde(default = "default_port")]
    pub(super) port: u16,
    #[serde(default = "default_address")]
    pub(super) address: IpAddr,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub(super) struct TelegramConfig {
    pub(super) token: String,
    #[serde(default)]
    pub(super) send_log: SendLog,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SendLog {
    Never,
    Always,
    Success,
    Failure,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Token {
    pub(super) secret: String,
    pub(super) access: HashSet<String>,
}

#[inline]
fn default_port() -> u16 {
    DEFAULT_PORT
}

#[inline]
fn default_address() -> IpAddr {
    IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))
}

#[inline]
fn default_user() -> String {
    String::from(DEFAULT_USER)
}

#[inline]
fn default_group() -> String {
    String::from(DEFAULT_GROUP)
}

impl Serialize for Tokens {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Tokens {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Tokens(Deserialize::deserialize(deserializer)?))
    }
}

impl Config {
    pub fn user(&self) -> &str {
        &self.main.user
    }

    pub fn group(&self) -> &str {
        &self.main.group
    }

    pub fn get_token(&self, token: &str) -> Option<&Token> {
        self.tokens.0.get(token)
    }

    pub fn port(&self) -> u16 {
        self.main.listen.port
    }

    pub fn address(&self) -> &IpAddr {
        &self.main.listen.address
    }
}

impl Token {
    pub fn can_access(&self, project: &str) -> bool {
        self.access.contains(project)
    }

    pub fn secret(&self) -> &str {
        &self.secret
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
    pub fn should_send(self, successful: bool) -> bool {
        match self {
            SendLog::Always => true,
            SendLog::Success if successful => true,
            SendLog::Failure if !successful => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::net::IpAddr;

    #[test]
    fn test_should_send_log() {
        assert_eq!(true, SendLog::Always.should_send(true));
        assert_eq!(true, SendLog::Always.should_send(false));
        assert_eq!(true, SendLog::Success.should_send(true));
        assert_eq!(false, SendLog::Success.should_send(false));
        assert_eq!(false, SendLog::Failure.should_send(true));
        assert_eq!(true, SendLog::Failure.should_send(false));
        assert_eq!(false, SendLog::Never.should_send(true));
        assert_eq!(false, SendLog::Never.should_send(false));
    }

    #[test]
    fn test_can_access() {
        let mut token = Token {
            secret: "foo_bar_baz".into(),
            access: HashSet::new(),
        };

        token.access.insert("test".into());

        assert_eq!(true, token.can_access("test"));
        assert_eq!(false, token.can_access("lorem_ipsum"));
    }

    #[test]
    fn test_get_token() {
        let mut tokens = HashMap::new();

        tokens.insert(
            "travis".into(),
            Token {
                secret: "foo".into(),
                access: HashSet::new(),
            },
        );

        let config = Config {
            tokens: Tokens(tokens),
            ..Default::default()
        };

        assert_eq!(
            Some(&Token {
                secret: "foo".into(),
                access: HashSet::new(),
            }),
            config.get_token("travis")
        )
    }

    #[test]
    fn test_listen_config() {
        let config = Config {
            main: MainConfig {
                listen: ListenConfig {
                    port: 1234,
                    address: "172.16.16.16".parse().unwrap(),
                },
                ..Default::default()
            },
            ..Default::default()
        };

        assert_eq!(1234, config.port());
        assert_eq!(&"172.16.16.16".parse::<IpAddr>().unwrap(), config.address());
    }

    #[test]
    fn test_user_group() {
        let config = Config {
            main: MainConfig {
                user: String::from("user"),
                group: String::from("group"),
                ..Default::default()
            },
            ..Default::default()
        };

        assert_eq!("user", config.user());
        assert_eq!("group", config.group());
    }
}
