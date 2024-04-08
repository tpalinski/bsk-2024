use rsa::{pkcs1::DecodeRsaPublicKey, pkcs8::DecodePrivateKey, RsaPrivateKey, RsaPublicKey};

use crate::http_client::{privkey, pubkey};

mod aes;

pub async fn get_private_key(author: String, token: String, pin: String) -> Result<(RsaPrivateKey, String), String> {
    let res = match privkey(author, token).await {
        Ok(e) => e,
        Err(status) => {
            return Err(format!("Error while fetching private key: {status}"))
        }
    };
    let key_string = aes::decrypt_key(res.key, pin)?;
    let key = RsaPrivateKey::from_pkcs8_pem(&key_string).expect("Error while parsing private key");
    Ok((key, res.token))
}

pub async fn get_public_key(author: String) -> Result<RsaPublicKey, String> {
    let key_string = match pubkey(author).await {
        Ok(key) => key.key,
        Err(status) => return Err(format!("Error while fetching public key: {status}"))
    };
    let key = RsaPublicKey::from_pkcs1_pem(&key_string).expect("Error while parsing public key");
    Ok(key)
}
