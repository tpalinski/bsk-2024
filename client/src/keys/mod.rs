mod aes;

pub static PRIVATE_KEY: &'static str = "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";
pub static PUBLIC_KEY: &'static str = "-----BEGIN RSA PUBLIC KEY-----\nMIICCgKCAgEAoUVPjNa9wT+0ACOag7E8ZzyjyrxNVVkv1Eh7SQCkZXXW+dHzf9C0\nZLjl166BnTbAcHEUL6wV4fT59W1LOJHKOBqU9JwfnFp7L7Zmaxo6XDDWtD10OE8+\nAlBFOTrOLnZ7/GwRjoxWFs2xsNxhWUTV9ZP1lUw39+KrQu4a8LqQwYUq60Ioujrl\nQl/K5tMT50b2aeAw7Qa1G1anH8/mpGdcdk6BR7nDNjzX8QJuvG+rZPhCR3CQqCsy\nkBKLD1H7fnsbY7Qwq/aw64l6XYyf5PBhPpL1kkz0ze6buOL3DsmYmz3UEBEOg3MQ\n23yRcm5+0pFTut67wgiV5CvrBnNoD5DKQZsFTewIWQhQKCG0tMI/u7GYN8OHfu47\nfAxTxfitJjYZb2ImJJdeofhTEEbCvTzmYiiyfffJRFn4dI/6Nqog5Q1KDiVT9UBZ\nGarot84wj3pM3CsRWpApe8hIiAQ6yXOsU2cDCTwMVmk/TPgAYkPHCy7vwMJElzv9\nmko/ucFDh3QfVZ+1JqXjXdex56lC0xKN8yQE44N22YVnDcGaIJT/TKHFpc9sIcP/\nY45Dtb4m3mHY6+oXQWleCMdNhx8mUEg2NcD88sm864DH6XF21vTqCNkAwsItZMdg\nmOW6URWLeHF52WFuGRsmh3PdLHKE4jKzC6PXvg/FSRPj7AK8YebkpEsCAwEAAQ==\n-----END RSA PUBLIC KEY-----\n";
pub static USER_PIN: &'static str = "test";

pub fn get_keys() -> String {
    // TODO - actual key fetching
    aes::decrypt_key(PRIVATE_KEY.to_owned(), USER_PIN.to_owned())
}
