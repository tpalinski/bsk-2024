use aes_gcm::{aead::Aead, Aes256Gcm, Key, KeyInit, Nonce};
use base64ct::{Base64, Encoding};
use sha2::{Digest, Sha256};

pub fn decrypt_key(key: String, pin: String) -> Result<String, String> {
    let ciphertext: Vec<u8> = Base64::decode_vec(&key).unwrap();
    let (nonce, ciphertext) = split_ciphertext(ciphertext);
    let nonce: &[u8] = &nonce;
    let nonce = Nonce::clone_from_slice(nonce);
    let ciphertext: &[u8] = &ciphertext;
    let aes_key = generate_aes_key(&pin);
    let cipher = Aes256Gcm::new(&aes_key);
    let decrypted = match cipher.decrypt(&nonce, ciphertext) {
        Ok(s) => s,
        Err(_) => return Err("Error while decrypting private key - invalid pin".to_owned())
    };
    Ok(String::from_utf8(decrypted).unwrap())
}

fn generate_aes_key(pin: &str) -> Key<Aes256Gcm> {
    let hashed = Sha256::digest(pin);
    hashed.into()
}

fn split_ciphertext(ciphertext: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    let nonce = Vec::from(&ciphertext[..12]);
    let cipher = Vec::from(&ciphertext[12..]);
    (nonce, cipher)
}


