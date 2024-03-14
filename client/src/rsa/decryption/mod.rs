use base64ct::{Base64, Encoding};
use rsa::pkcs1v15::Signature;
use rsa::{pkcs1v15::VerifyingKey, RsaPublicKey};
use rsa::sha2::Sha256;
use rsa::signature::Verifier;

pub fn verify_signature(data: Vec<u8>, key: RsaPublicKey, signature: String) -> Result<(), rsa::signature::Error> {
    let ver_key = VerifyingKey::<Sha256>::new(key);
    let signature_bytes: &[u8] = &Base64::decode_vec(&signature).expect("Error while decoding signature");
    let signature  = Signature::try_from(signature_bytes).expect("Error while deserializing signature");
    let res = ver_key.verify(&data, &signature);
    res
}
