use std::collections::{HashMap, HashSet};
use std::net::{IpAddr, Ipv4Addr};

const DEFAULT_PORT: u16 = 8629;

#[derive(Debug, Clone, Default)]
pub struct Config {
    main: MainConfig,
    tokens: HashMap<String, Token>,
    projects: HashMap<String, Project>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
struct Project {
    scripts: Vec<Script>,
    #[serde(default)]
    environment: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
struct Script {
    command: Vec<String>,
    #[serde(default)]
    allow_failure: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(deny_unknown_fields)]
struct MainConfig {
    #[serde(default)]
    listen: ListenConfig,
    telegram: Option<TelegramConfig>,
    tls: Option<TlsConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
struct ListenConfig {
    #[serde(default = "default_port")]
    port: u16,
    #[serde(default = "default_address")]
    address: IpAddr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
struct TelegramConfig {
    token: String,
    #[serde(default)]
    send_log: SendLog,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum SendLog {
    Never,
    Always,
    Success,
    Failure,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
struct TlsConfig {
    certificate: String,
    certificate_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Token {
    secret: String,
    access: HashSet<String>,
}

#[inline]
fn default_port() -> u16 {
    DEFAULT_PORT
}

#[inline]
fn default_address() -> IpAddr {
    IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))
}

impl Config {
    pub fn get_token(&self, token: &str) -> Option<&Token> {
        self.tokens.get(token)
    }

    pub fn port(&self) -> u16 {
        self.main.listen.port
    }

    pub fn address(&self) -> &IpAddr {
        &self.main.listen.address
    }
}

impl TlsConfig {
    fn certificate(&self) -> &str {
        &self.certificate
    }

    fn certificate_key(&self) -> &str {
        &self.certificate_key
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
            tokens: tokens,
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
    fn test_tls_config() {
        let config = Config {
            main: MainConfig {
                tls: Some(TlsConfig {
                    certificate: "cert.pem".into(),
                    certificate_key: "key.pem".into(),
                }),
                ..Default::default()
            },
            ..Default::default()
        };

        assert_eq!(Some("cert.pem"), config.tls().map(|tls| tls.certificate()));

        assert_eq!(
            Some("key.pem"),
            config.tls().map(|tls| tls.certificate_key())
        );
    }
}
