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

pub struct OsPathImpl<'a> {
    context: &'a Context,
}

impl<'a> OsPathImpl<'a> {
    fn scripts_path(&self) -> PathBuf {
        path!(self.context.config_path(), SCRIPTS_DIR)
    }
}

impl<'a> OsPath for OsPathImpl<'a> {
    fn path(&self) -> OsString {
        let path = env::var_os("PATH");

        path.map(|path| extend_path(&path, self.scripts_path()).unwrap())
            .unwrap_or_else(|| self.scripts_path().into())
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
