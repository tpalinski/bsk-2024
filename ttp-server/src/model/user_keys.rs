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

    pub fn keys(self) -> (String, String) {
        (self.public_key, self.private_key)
    }
}

