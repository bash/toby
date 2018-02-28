use std::fs::File;
use std::io::{self, Read, Write};

fn main() {
    let mut buffer = Vec::new();
    let mut file = match File::open("Build.txt") {
        Ok(file) => file,
        Err(_) => {
            panic!(
                "

Build is not properly configured (`Build.txt` is missing).
Did you forget to run `./configure`?

"
            );
        }
    };

    file.read_to_end(&mut buffer).unwrap();

    io::stdout().write_all(&buffer).unwrap();

    println!("cargo:rerun-if-changed=Build.txt")
}
