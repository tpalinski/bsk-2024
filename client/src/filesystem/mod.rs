use std::{fs, path::PathBuf};

pub fn get_file_contents(path: PathBuf) -> Vec<u8> {
    let content = fs::read(dbg!(path)).expect("Error: could not read the content of the file");
    content
}

pub fn save_to_file(data: Vec<u8>, path: PathBuf) {
    fs::write(path, data).expect("Error while writing to file");
}
