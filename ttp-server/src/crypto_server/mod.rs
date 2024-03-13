use aes_gcm::{aead::Aead, AeadCore, Aes256Gcm, Key, KeyInit};
use base64ct::{Base64, Encoding};
use rand::rngs::OsRng;
use rsa::{pkcs1::EncodeRsaPublicKey, pkcs8::EncodePrivateKey, RsaPrivateKey, RsaPublicKey};
use sha2::{Digest, Sha256};

use crate::model::user_keys::UserKeys;


pub fn generate_keys(pin: &str) -> UserKeys{
    let (public_key, private_key) = generate_rsa_keys();
    let hashed_private_key = encrypt_private_key(pin, &private_key);
    let public_key = public_key.to_pkcs1_pem(rsa::pkcs8::LineEnding::LF).unwrap();
    UserKeys::new(public_key, hashed_private_key)
}

fn generate_rsa_keys() -> (RsaPublicKey, RsaPrivateKey) {
    let mut rng = rand::thread_rng();
    let bits = 4096;

    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Error while creating private key");
    let public_key = RsaPublicKey::from(&private_key);
    (public_key, private_key)
}

fn encrypt_private_key(pin: &str, key: &RsaPrivateKey) -> String {
    println!("Encrypting private key...");
    let pin_hash = generate_aes_key(pin);
    let cipher = Aes256Gcm::new(&pin_hash);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let key_as_bytes = key.to_pkcs8_pem(rsa::pkcs8::LineEnding::LF).unwrap();
    let key_as_bytes = key_as_bytes.as_bytes();
    let mut encrypted_key = cipher.encrypt(&nonce, key_as_bytes).expect("Error while encrypting private key");
    println!("Private key encrypted successfully");
    let mut nonce: Vec<u8> = nonce.into_iter().collect();
    nonce.append(&mut encrypted_key);
    Base64::encode_string(&nonce)
}

fn generate_aes_key(pin: &str) -> Key<Aes256Gcm> {
    let hashed = Sha256::digest(pin);
    hashed.into()
}
