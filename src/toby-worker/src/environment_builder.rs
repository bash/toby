use std::ffi::OsString;

pub trait EnvironmentBuilder {
    fn get_path(&self) -> OsString;
}
