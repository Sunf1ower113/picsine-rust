use std::fs::OpenOptions;
use std::io::Write;

pub fn open_or_create(file: &str, content: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file).unwrap();
    file.write_all(content.as_ref()).unwrap();
}