use base64ct::{Base64, Encoding};
use sha2::{Digest, Sha256};

pub fn get_hash(data: &[u8]) -> String {
    let hash = Sha256::digest(data);
    Base64::encode_string(&hash)
}
