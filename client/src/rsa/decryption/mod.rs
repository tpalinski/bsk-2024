use base64ct::{Base64, Encoding};
use rsa::pkcs1v15::Signature;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey};
use rsa::{pkcs1v15::VerifyingKey, RsaPublicKey};
use rsa::sha2::Sha256;
use rsa::signature::Verifier;

use crate::filesystem::split_data;

pub fn verify_signature(data: &[u8], key: RsaPublicKey, signature: String) -> Result<(), rsa::signature::Error> {
    let ver_key = VerifyingKey::<Sha256>::new(key);
    let signature_bytes = match Base64::decode_vec(&signature) {
        Ok(s) => s,
        Err(_) => return Err(rsa::signature::Error::new())
    };
    let signature  = Signature::try_from(signature_bytes.as_slice())?;
    let res = ver_key.verify(data, &signature);
    res
}

pub fn decrypt(data: &[u8], key: RsaPrivateKey) -> Result<Vec<u8>, String> {
    let split_data = split_data(data, 512);
    let mut out: Vec<u8> = Vec::new();
    for chunk in split_data {
        let mut decrypted = decrypt_block(chunk, &key)?;
        out.append(&mut decrypted);
    }
    Ok(out)
}

fn decrypt_block(data: &[u8], key: &RsaPrivateKey) -> Result<Vec<u8>, String> {
    let plain = key.decrypt(Pkcs1v15Encrypt, data);
    match plain {
        Ok(bytes) => Ok(bytes),
        Err(e) => Err(format!("Error while decrypting file: {e}"))
    }
}
