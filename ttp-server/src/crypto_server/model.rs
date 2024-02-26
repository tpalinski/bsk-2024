use rsa::{pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey}, RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct UserKeys {
    public_key: String,
    private_key: String
}

impl UserKeys {
    pub fn new(public_key: String, private_key: String) -> Self {
        UserKeys{public_key, private_key}
    }

    pub fn from_keys(public_key: RsaPublicKey, private_key: RsaPrivateKey) -> Self {
        let public_key = public_key.to_pkcs1_pem(rsa::pkcs8::LineEnding::LF).unwrap();
        let private_key = private_key.to_pkcs1_pem(rsa::pkcs8::LineEnding::LF).unwrap().to_string();
        UserKeys::new(public_key, private_key)
    }

    pub fn keys(self) -> (String, String) {
        (self.public_key, self.private_key)
    }
}

