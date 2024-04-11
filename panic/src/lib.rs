use std::fs::File;

pub fn open_file(s: &str) -> File {
    let msg = File::open(s);
    msg.expect("File not found")
}