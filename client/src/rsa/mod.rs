use crate::keys::get_private_key;

mod encryption;

pub fn encrypt_data(data: Vec<u8>) -> String {
    let key = get_private_key();
    let encrypted = encryption::encrypt(data, key);
    encrypted
}
