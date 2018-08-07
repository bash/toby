use tempdir::TempDir;
use toby_core::Context;

pub struct TempContext {
    #[allow(dead_code)]
    config_dir: TempDir,
    #[allow(dead_code)]
    log_dir: TempDir,
    #[allow(dead_code)]
    runtime_dir: TempDir,
    context: Context,
}

impl std::ops::Deref for TempContext {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl TempContext {
    pub fn create() -> std::io::Result<Self> {
        let config_dir = tempdir::TempDir::new("toby")?;
        let log_dir = tempdir::TempDir::new("toby")?;
        let runtime_dir = tempdir::TempDir::new("toby")?;

        let context = Context::new(
            config_dir.path().to_owned(),
            log_dir.path().to_owned(),
            runtime_dir.path().to_owned(),
        );

        Ok(Self {
            context,
            config_dir,
            log_dir,
            runtime_dir,
        })
    }
}
