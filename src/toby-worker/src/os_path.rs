use std::env;
use std::env::{join_paths, split_paths, JoinPathsError};
use std::ffi::{OsStr, OsString};
use std::iter;
use std::path::PathBuf;
use toby_core::{path, Context};

const SCRIPTS_DIR: &str = "scripts.d";

pub trait OsPath {
    fn path(&self) -> OsString;
}

pub struct ExtendingPath<'a> {
    context: &'a Context,
}

impl<'a> OsPath for ExtendingPath<'a> {
    fn path(&self) -> OsString {
        let path = env::var_os("PATH");
        let scripts_path = path!(self.context.config_path(), SCRIPTS_DIR);

        extend_script_path(path.as_ref(), scripts_path)
    }
}

fn extend_script_path(path: Option<&OsString>, scripts_path: PathBuf) -> OsString {
    match path {
        Some(path) => extend_path(&path, scripts_path).expect("Failed to extend path"),
        None => scripts_path.into(),
    }
}

fn extend_path<T>(path: &T, extension: PathBuf) -> Result<OsString, JoinPathsError>
where
    T: AsRef<OsStr>,
{
    join_paths(split_paths(path).chain(iter::once(extension)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_extend_path_works() {
        assert_eq!(
            join_paths(&["/foo", "/bar", "/etc/toby/scripts.d"]).unwrap(),
            extend_path(
                &join_paths(&["/foo", "/bar"]).unwrap(),
                PathBuf::from("/etc/toby/scripts.d")
            ).unwrap()
        );
    }
}
