#![feature(non_exhaustive)]
#![warn(rust_2018_idioms)]

use self::config::ConfigLoader;
use self::job::Hook as JobHook;
use std::error;
use std::fmt;

pub mod config;
pub mod job;

pub struct Context<'a> {
    pub registry: &'a mut dyn Registry,
    pub config_loader: ConfigLoader<'a>,
}

pub trait Registry {
    fn register_job_hook(&mut self, hook: Box<dyn JobHook>);
}

#[non_exhaustive]
#[derive(Debug)]
pub enum RegistrarError {
    ConfigError(config::Error),
    Custom(Box<dyn error::Error>),
}

impl fmt::Display for RegistrarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegistrarError::ConfigError(err) => err.fmt(f),
            RegistrarError::Custom(err) => err.fmt(f),
        }
    }
}

impl error::Error for RegistrarError {}

impl From<config::Error> for RegistrarError {
    fn from(err: config::Error) -> Self {
        RegistrarError::ConfigError(err)
    }
}
