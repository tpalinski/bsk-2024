use base64ct::{Base64, Encoding};
use rsa::{Pkcs1v15Encrypt, RsaPublicKey};
use rsa::{pkcs1v15::SigningKey, RsaPrivateKey};
use rsa::sha2::Sha256;
use rsa::signature::{RandomizedSigner, SignatureEncoding};

use crate::filesystem::split_data;

use super::SPLIT_BLOCK_SIZE;

pub fn sign(content: &[u8], key: RsaPrivateKey) -> String{
    let mut rng = rand::thread_rng();
    let signing_key = SigningKey::<Sha256>::new(key);
    let signature = signing_key.sign_with_rng(&mut rng, content).to_bytes();
    let sig = Base64::encode_string(&signature);
    sig
}

pub fn encrypt(data: &[u8], key: RsaPublicKey) -> Result<String, String> {
    let mut rng = rand::thread_rng();
    // Split data
    let split_data = split_data(data, SPLIT_BLOCK_SIZE);
    let mut out_data: Vec<u8> = Vec::new();
    for chunk in split_data {
        let mut cipher = match key.encrypt(&mut rng, Pkcs1v15Encrypt, chunk) {
            Ok(c) => c,
            Err(e) => return Err(format!("Error while encrypting data: {e}"))
        };
        out_data.append(&mut cipher);
    }
    let enc = Base64::encode_string(&out_data);
    Ok(enc)
}
