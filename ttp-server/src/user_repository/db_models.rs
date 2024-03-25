use serde::{Deserialize, Serialize};

use crate::model::user_keys::UserKeys;

// Information about user keys
#[derive(Serialize, Debug, Deserialize)]
pub struct UserKeyData {
    private_key: String,
    public_key: String
}

impl From<UserKeys> for UserKeyData {
    fn from(value: UserKeys) -> Self {
        let (public_key, private_key) = value.keys();
        UserKeyData { private_key, public_key }
    }
}

impl UserKeyData {
    pub fn new() -> Self {
        UserKeyData{private_key: String::new(), public_key: String::new()}
    }

    pub fn pubkey(&self) -> String {
        self.public_key.to_owned()
    }
}

// User login data
#[derive(Serialize, Debug, Deserialize)]
pub struct User {
    password: String,
    token: String,
}


impl User {
    pub fn new(password: String, token: String) -> Self {
        User{password, token}
    }
}
