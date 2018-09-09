#![feature(decl_macro)]

use self::context::TempContext;
use std::fs::{create_dir, OpenOptions};
use std::io::Write;
use std::net::{IpAddr, Ipv4Addr};
use toby_core::config::ConfigLoader;
use toby_core::path;

mod context;

#[test]
fn test_load_config_works() {
    let context = TempContext::create().unwrap();
    let config_path = context.config_path();

    macro write_file($path:expr, $contents:expr) {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open($path)
            .expect("unable to open file");

        file.write_all($contents).expect("unable to write file");
    };

    write_file!(
        path!(config_path, "toby.toml"),
        br#"[listen]
address = "172.16.16.16"
port = 1234"#
    );

    write_file!(
        path!(config_path, "tokens.toml"),
        br#"[robot]
secret = "much_secret_wow"
access = ["dreams"]"#
    );

    let projects_path = path!(config_path, "projects.d");
    create_dir(&projects_path).expect("unable to create dir");

    write_file!(
        path!(&projects_path, "dreams.toml"),
        br#"[[scripts]]
command = ["systemctl", "restart", "dreams"]
allow_failure = true"#
    );

    let config = ConfigLoader::new(&context).load().unwrap();

    assert_eq!(
        &IpAddr::V4(Ipv4Addr::new(172, 16, 16, 16)),
        config.address()
    );

    assert_eq!(1234, config.port());

    assert_eq!(
        "much_secret_wow",
        config.get_token("robot").unwrap().secret()
    );
    assert!(config.get_token("robot").unwrap().can_access("dreams"));
}
