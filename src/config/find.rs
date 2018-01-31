use std::fs::read_dir;
use std::path::PathBuf;
use std::io;

const LOCAL_CONFIG_DIR: &'static str = env!("TOBY_CONFIG_PATH");

const CONFIG_EXTENSION: &'static str = "toml";
const PROJECT_CONFIG_PATH: &'static str = "toby/conf.d";
const CONFIG_PATH: &'static str = "toby/toby.toml";
const TOKENS_PATH: &'static str = "toby/tokens.toml";

fn is_config_file(path: &PathBuf) -> bool {
    if !path.is_file() {
        return false;
    }

    match path.extension() {
        Some(extension) => extension == CONFIG_EXTENSION,
        None => false,
    }
}

fn prefix_path(path: &str) -> PathBuf {
    let mut prefixed_path = PathBuf::from(LOCAL_CONFIG_DIR);
    prefixed_path.push(path);
    prefixed_path
}

pub fn find_project_configs() -> io::Result<Vec<PathBuf>> {
    let mut files = vec![];
    let path = prefix_path(PROJECT_CONFIG_PATH);

    if path.exists() {
        for entry in read_dir(path)? {
            let path = entry?.path();

            if is_config_file(&path) {
                files.push(path);
            }
        }
    }

    return Ok(files);
}

pub fn find_config_file() -> PathBuf {
    prefix_path(CONFIG_PATH)
}

pub fn find_tokens_file() -> PathBuf {
    prefix_path(TOKENS_PATH)
}
