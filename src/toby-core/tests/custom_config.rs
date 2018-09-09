#![feature(decl_macro)]

#[macro_use]
extern crate serde_derive;

use self::context::TempContext;
use std::fs::OpenOptions;
use std::io::Write;
use toby_core::config::ConfigLoader;
use toby_core::path;

mod context;

#[test]
fn test_load_config_works() {
    let context = TempContext::create().unwrap();
    let config_path = context.config_path();

    #[derive(Debug, Deserialize, Eq, PartialEq)]
    struct ExampleConfig {
        foo: bool,
    }

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(path!(config_path, "example.toml"))
        .expect("unable to open file");

    file.write_all(br#"foo = true"#)
        .expect("unable to write file");

    let config = ConfigLoader::new(&context).load_custom("example").unwrap();

    assert_eq!(ExampleConfig { foo: true }, config);
}
