use std::fs::File;
use std::io;

fn main() {
    let mut file = match File::open("../Build.txt") {
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

    io::copy(&mut file, &mut io::stdout()).unwrap();

    println!("cargo:rerun-if-changed=Build.txt")
}
