use std::fs::read_dir;
use std::path::PathBuf;
use std::io;

const CONFIG_EXTENSION: &'static str = "toml";

#[cfg(debug_assertions)]
const CONFIG_PATHS: [&'static str; 3] = [
    "./conf/toby.toml",
    "/etc/toby/toby.toml",
    "/usr/local/etc/toby/toby.toml",
];

#[cfg(not(debug_assertions))]
const CONFIG_PATHS: [&'static str; 2] = ["/etc/toby/toby.toml", "/usr/local/etc/toby/toby.toml"];

lazy_static! {
    static ref CONFIG_DIRS: Vec<PathBuf> = vec![
        PathBuf::from("/etc/toby/conf.d"),
        PathBuf::from("/usr/local/etc/toby/conf.d"),
        #[cfg(debug_assertions)] PathBuf::from("./conf/conf.d"),
    ];
}

fn is_config_file(path: &PathBuf) -> bool {
    if !path.is_file() {
        return false;
    }

    match path.extension() {
        Some(extension) => extension == CONFIG_EXTENSION,
        None => false,
    }
}

pub fn find_project_configs() -> io::Result<Vec<PathBuf>> {
    let config_dirs = CONFIG_DIRS.iter().filter(|path| path.exists());
    let mut files = vec![];

    for config_dir in config_dirs {
        for entry in read_dir(config_dir)? {
            let path = entry?.path();

            if is_config_file(&path) {
                files.push(path);
            }
        }
    }

    return Ok(files);
}

pub fn find_config_file() -> Option<&'static str> {
    CONFIG_PATHS
        .iter()
        .find(|path| PathBuf::from(path).exists())
        .map(|path| *path)
}
