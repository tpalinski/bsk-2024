use base64ct::{Base64, Encoding};
use rsa::{pkcs1v15::SigningKey, RsaPrivateKey};
use rsa::sha2::Sha256;
use rsa::signature::{RandomizedSigner, SignatureEncoding};

pub fn sign(content: &[u8], key: RsaPrivateKey) -> String{
    let mut rng = rand::thread_rng();
    let signing_key = SigningKey::<Sha256>::new(key);
    let signature = signing_key.sign_with_rng(&mut rng, content).to_bytes();
    let sig = Base64::encode_string(&signature);
    sig
}
