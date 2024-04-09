use crate::keys::{get_private_key, get_public_key};

use self::{encryption::encrypt, model::{Signature, SignatureProps}};

mod encryption;
mod decryption;
mod cert;
mod utils;
mod model;

pub async fn sign_data(data: &[u8], author: String, token: String, pin: String) -> Result<(String, String), String> {
    let (key, new_token) = get_private_key(author.clone(), token, pin).await?;
    let props = SignatureProps{
        author,
        data,
        pkey: key
    };
    let encrypted = Signature::generate(props);
    Ok((encrypted.to_xml(), new_token))
}

pub async fn verify_signature(data: &[u8], xades_data: Vec<u8>) -> Result<bool, String> {
    let signature = Signature::from_xml(xades_data)?;
    signature.verify(data).await
}

pub async fn encrypt_data(data: &[u8], user: String) -> Result<String, String> {
    let key = get_public_key(user).await?;
    encrypt(data, key)
}
