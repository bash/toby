use std::{error, fmt};

#[derive(Debug)]
pub struct Error(toby_core::config::ConfigError);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl error::Error for Error {}

pub struct ConfigLoader<'a>(&'a toby_core::config::ConfigLoader<'a>);

impl<'a> ConfigLoader<'a> {
    #[doc(hidden)]
    pub fn new(inner: &'a toby_core::config::ConfigLoader<'a>) -> Self {
        ConfigLoader(inner)
    }

    pub fn load_custom<T>(&self, name: &str) -> Result<T, Error>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        self.0.load_custom(name).map_err(Error)
    }
}
