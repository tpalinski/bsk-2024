use std::{fs, path::PathBuf};

pub fn get_file_contents(path: PathBuf) -> Vec<u8> {
    let content = fs::read(path).expect("Error: could not read the content of the file");
    content
}
