use rsa::{pkcs1::DecodeRsaPublicKey, pkcs8::DecodePrivateKey, RsaPrivateKey, RsaPublicKey};

use crate::http_client::privkey;

mod aes;

pub static PUBLIC_KEY: &'static str = "-----BEGIN RSA PUBLIC KEY-----\nMIICCgKCAgEAoUVPjNa9wT+0ACOag7E8ZzyjyrxNVVkv1Eh7SQCkZXXW+dHzf9C0\nZLjl166BnTbAcHEUL6wV4fT59W1LOJHKOBqU9JwfnFp7L7Zmaxo6XDDWtD10OE8+\nAlBFOTrOLnZ7/GwRjoxWFs2xsNxhWUTV9ZP1lUw39+KrQu4a8LqQwYUq60Ioujrl\nQl/K5tMT50b2aeAw7Qa1G1anH8/mpGdcdk6BR7nDNjzX8QJuvG+rZPhCR3CQqCsy\nkBKLD1H7fnsbY7Qwq/aw64l6XYyf5PBhPpL1kkz0ze6buOL3DsmYmz3UEBEOg3MQ\n23yRcm5+0pFTut67wgiV5CvrBnNoD5DKQZsFTewIWQhQKCG0tMI/u7GYN8OHfu47\nfAxTxfitJjYZb2ImJJdeofhTEEbCvTzmYiiyfffJRFn4dI/6Nqog5Q1KDiVT9UBZ\nGarot84wj3pM3CsRWpApe8hIiAQ6yXOsU2cDCTwMVmk/TPgAYkPHCy7vwMJElzv9\nmko/ucFDh3QfVZ+1JqXjXdex56lC0xKN8yQE44N22YVnDcGaIJT/TKHFpc9sIcP/\nY45Dtb4m3mHY6+oXQWleCMdNhx8mUEg2NcD88sm864DH6XF21vTqCNkAwsItZMdg\nmOW6URWLeHF52WFuGRsmh3PdLHKE4jKzC6PXvg/FSRPj7AK8YebkpEsCAwEAAQ==\n-----END RSA PUBLIC KEY-----\n";
pub static USER_PIN: &'static str = "test";

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

pub fn get_public_key() -> RsaPublicKey {
    let key_string = RsaPublicKey::from_pkcs1_pem(PUBLIC_KEY).expect("Error while parsing public key");
    key_string
}
