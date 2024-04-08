use base64ct::{Base64, Encoding};
use rsa::pkcs1v15::Signature;
use rsa::{pkcs1v15::VerifyingKey, RsaPublicKey};
use rsa::sha2::Sha256;
use rsa::signature::Verifier;

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
