use std::{fs, path::PathBuf};

fn get_file_contents(path: PathBuf) -> Vec<u8> {
    let content = fs::read(path).expect("Error: could not read the content of the file");
    content
}

fn save_to_file(data: Vec<u8>, path: PathBuf) {
    fs::write(path, data).expect("Error while writing to file");
}

fn convert_to_home(fake_path: String) -> PathBuf {
    let file_name = fake_path.rsplit('\\').collect::<Vec<&str>>()[0].to_owned();
    let mut path = dirs::home_dir().unwrap();
    path.push(&file_name);
    path
}

fn convert_to_sig_path(fake_path: String) -> PathBuf {
    let file_name = fake_path.rsplit('\\').collect::<Vec<&str>>()[0].to_owned() + ".xades";
    let mut path = dirs::home_dir().unwrap();
    path.push(&file_name);
    path
}

fn convert_from_sig_path(xades_path: &PathBuf) -> PathBuf {
    let file_name = xades_path.file_stem().unwrap();
    let mut res = xades_path.clone();
    res.set_file_name(file_name);
    res
}

pub fn get_data_from_fake_path(fake_path: String) -> Vec<u8> {
    let path = convert_to_home(fake_path);
    let data = get_file_contents(path);
    data
}

pub fn save_to_fake_path(data: Vec<u8>, fake_path: String) {
    let sig_path = convert_to_sig_path(fake_path);
    save_to_file(data, sig_path);
}

pub fn get_verify_data(fake_path: String) -> (Vec<u8>, Vec<u8>) {
    let xades_path = convert_to_home(fake_path);
    let file_path = convert_from_sig_path(&xades_path);
    let xades_data = get_file_contents(xades_path);
    let claim_data = get_file_contents(file_path);
    (xades_data, claim_data)
}
