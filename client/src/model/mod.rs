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
pub struct TokenResponse {
    pub token: String
}
