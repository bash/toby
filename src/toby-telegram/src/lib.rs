use std::error::Error;
use toby_plugin::job::{Hook, Job, Outcome};
use toby_plugin::{Context, RegistrarError};

#[macro_use]
extern crate serde_derive;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct TelegramConfig {
    token: String,
    chat_id: i64,
}

struct Telegram {
    #[allow(dead_code)]
    config: TelegramConfig,
}

impl Hook for Telegram {
    fn before_job(&self, _job: &'_ Job) -> Result<(), Box<dyn Error>> {
        unimplemented!();
    }

    fn after_job(&self, _job: &'_ Job, _outcome: Outcome) -> Result<(), Box<dyn Error>> {
        unimplemented!();
    }
}

#[no_mangle]
pub fn plugin_registrar(context: &mut Context) -> Result<(), RegistrarError> {
    let config = context.config_loader.load_custom("telegram")?;

    context
        .registry
        .register_job_hook(Box::new(Telegram { config }));

    Ok(())
}
