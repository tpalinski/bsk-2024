use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoginRequest{
    pub email: String,
    pub password: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoginResponse {
    pub email: String,
    pub token: String,
    pub name: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PrivkeyRequest {
    pub email: String,
    pub token: String,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PrivkeyResponse {
    pub key: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PubkeyResponse {
    pub key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TokenResponse {
    pub token: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmailRequest {
    pub email: String
}
