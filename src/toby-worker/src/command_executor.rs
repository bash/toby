use std::collections::HashMap;
use std::ffi::{OsStr, OsString};

pub struct Command<'a> {
    pub command: Vec<&'a OsStr>,
    pub environment: &'a HashMap<&'a OsStr, OsString>,
}

pub trait CommandExecutor {
    fn execute_command(&self, _command: Command<'_>);
}
