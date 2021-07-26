use std::io;
use std::fs::File;

fn main() {
    let mut file = File::open("default.text")?;
    io::stdout().write_all(file);
}