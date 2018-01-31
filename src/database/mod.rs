use std::fs::File;
use std::io;
use toml;

const RUNTIME_DIR: Option<&'static str> = option_env!("RUNTIME_DIR");
const DEFAULT_RUNTIME_DIR: &'static str = "./conf/var/lib";

#[derive(Debug)]
pub struct Data {
    telegram_chat_ids: Vec<String>,
}

pub struct Database {
    file: File,
    data: Option<Data>,
}

impl Database {
    pub fn new() -> io::Result<Self> {
        let path = RUNTIME_DIR.unwrap_or(DEFAULT_RUNTIME_DIR);
        let file = File::open(path)?;

        Ok(Database { file, data: None })
    }

    // fn write() -> io::Result<()> {}
}
