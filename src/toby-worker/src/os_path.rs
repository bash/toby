use std::ffi::OsString;

pub trait OsPath {
    fn path(&self) -> OsString;
}
