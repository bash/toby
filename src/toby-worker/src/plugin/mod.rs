use libloading as lib;
use std::{error, fmt, io};
use toby_core::config::ConfigLoader;
use toby_plugin::config::ConfigLoader as PluginConfigLoader;
use toby_plugin::job::Hook as JobHook;
use toby_plugin::{Context as PluginContext, RegistrarError, Registry};

const PLUGIN_REGISTRAR_SYMBOL: &[u8] = b"plugin_registrar";

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Registrar(RegistrarError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(err) => err.fmt(f),
            Error::Registrar(err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<RegistrarError> for Error {
    fn from(err: RegistrarError) -> Error {
        Error::Registrar(err)
    }
}

pub struct LoadedPlugins {
    pub job_hooks: Vec<Box<dyn JobHook>>,
}

impl From<RegistryImpl> for LoadedPlugins {
    fn from(registry: RegistryImpl) -> LoadedPlugins {
        let RegistryImpl { job_hooks } = registry;

        LoadedPlugins { job_hooks }
    }
}

#[derive(Default)]
struct RegistryImpl {
    job_hooks: Vec<Box<dyn JobHook>>,
}

impl Registry for RegistryImpl {
    fn register_job_hook(&mut self, hook: Box<dyn JobHook>) {
        self.job_hooks.push(hook);
    }
}

fn library_path(name: &str) -> String {
    format!(
        "{}toby_{}{}",
        std::env::consts::DLL_PREFIX,
        name,
        std::env::consts::DLL_SUFFIX
    )
}

type RegistrarFn = fn(&mut PluginContext<'_>) -> Result<(), RegistrarError>;

pub fn load_plugins(
    plugins: &[String],
    config_loader: &'_ ConfigLoader<'_>,
) -> Result<LoadedPlugins, Error> {
    let mut registry = RegistryImpl::default();

    let mut context = PluginContext {
        registry: &mut registry,
        config_loader: PluginConfigLoader::new(config_loader),
    };

    for plugin in plugins {
        let lib = lib::Library::new(library_path(plugin))?;

        let registrar_fn: lib::Symbol<'_, RegistrarFn> =
            unsafe { lib.get(PLUGIN_REGISTRAR_SYMBOL)? };

        registrar_fn(&mut context)?;
    }

    Ok(LoadedPlugins::from(registry))
}
