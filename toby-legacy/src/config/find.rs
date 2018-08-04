use std::fs::read_dir;
use std::io;
use std::path::PathBuf;

const LOCAL_CONFIG_DIR: &str = env!("TOBY_CONFIG_PATH");

const CONFIG_EXTENSION: &str = "toml";
const PROJECT_CONFIG_PATH: &str = "conf.d";
const CONFIG_PATH: &str = "toby.toml";
const TOKENS_PATH: &str = "tokens.toml";

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

pub(crate) fn find_project_configs() -> io::Result<Vec<PathBuf>> {
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

    Ok(files)
}

pub(crate) fn find_config_file() -> PathBuf {
    prefix_path(CONFIG_PATH)
}

pub(crate) fn find_tokens_file() -> PathBuf {
    prefix_path(TOKENS_PATH)
}
