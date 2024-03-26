use std::{collections::BTreeMap, time::Instant};

use base64ct::{Base64, Encoding};
use sha2::{Digest, Sha256};
use jwt::SignWithKey;
use hmac::{Hmac, Mac};

static JWT_KEY: &[u8] = b"Totally secret JWT key hackers dont steal";


pub fn generate_token(email: String) -> String {
    let key: Hmac<Sha256> = Hmac::new_from_slice(JWT_KEY).expect("Error while creating JWT key");
    let mut claim: BTreeMap<String, String> = BTreeMap::new();
    claim.insert("email".to_owned(), email);
    claim.insert("time".to_owned(), format!("{:?}", Instant::now()));
    let token_str = claim.sign_with_key(&key).unwrap();
    token_str
}

pub fn check_passwords(plain: String, hashed: String) -> bool{
    let hash = Sha256::digest(plain);
    let stringified_hash = Base64::encode_string(&hash);
    stringified_hash == hashed
}
