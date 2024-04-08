use crate::keys::{get_private_key, get_public_key};

use self::model::{Signature, SignatureProps};

mod encryption;
mod decryption;
mod cert;
mod utils;
pub mod model;

pub async fn sign_data(data: &[u8], author: String, token: String) -> Result<String, String> {
    let (key, new_token) = get_private_key(author.clone(), token).await?;
    let props = SignatureProps{
        author,
        data,
        pkey: key
    };
    let encrypted = Signature::generate(props);
    Ok(encrypted.to_xml())
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
