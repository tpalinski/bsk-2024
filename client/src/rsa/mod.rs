use crate::keys::{get_private_key, get_public_key};

mod encryption;
mod decryption;

pub fn sign_data(data: Vec<u8>) -> String {
    let key = get_private_key();
    let encrypted = encryption::sign(data, key);
    encrypted
}

pub fn verify_signature(data: Vec<u8>, signature: String) -> Result<(), ()> {
    let key = get_public_key();
    let res = match decryption::verify_signature(data, key, signature) {
        Ok(_) => Ok(()),
        Err(e) => {
            dbg!(e);
            Err(())
        }
    };
    res
}
