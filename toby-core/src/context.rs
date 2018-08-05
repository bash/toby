use std::path::{Path, PathBuf};

const CONFIG_PATH: &str = env!("TOBY_CONFIG_PATH");
const LOG_PATH: &str = env!("TOBY_LOG_PATH");
const RUNTIME_PATH: &str = env!("TOBY_RUNTIME_PATH");

lazy_static! {
    static ref DEFAULT_CONTEXT: Context = {
        Context {
            config_path: PathBuf::from(CONFIG_PATH),
            log_path: PathBuf::from(LOG_PATH),
            runtime_path: PathBuf::from(RUNTIME_PATH),
        }
    };
}

#[derive(Debug)]
pub struct Context {
    config_path: PathBuf,
    log_path: PathBuf,
    runtime_path: PathBuf,
}

impl Context {
    pub fn new(config_path: PathBuf, log_path: PathBuf, runtime_path: PathBuf) -> Self {
        Context {
            config_path,
            log_path,
            runtime_path,
        }
    }

    ///
    /// Returns a context initialized with the path
    /// values defined at compile-time.
    ///
    pub fn default_context() -> &'static Self {
        &DEFAULT_CONTEXT
    }

    pub fn config_path(&self) -> &Path {
        self.config_path.as_path()
    }

    pub fn log_path(&self) -> &Path {
        self.log_path.as_path()
    }

    pub fn runtime_path(&self) -> &Path {
        self.runtime_path.as_path()
    }
}
