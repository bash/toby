use std::env::{join_paths, split_paths, JoinPathsError};
use std::ffi::{OsStr, OsString};
use std::iter;
use std::path::PathBuf;

pub(crate) fn extend_path<T>(path: &T, extension: PathBuf) -> Result<OsString, JoinPathsError>
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
