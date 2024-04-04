use std::sync::OnceLock;

use reqwest::{Client, StatusCode};

use crate::model::{LoginRequest, LoginResponse};

const BASE_URL: &str = "http://localhost:3000/api";

fn http_client() -> &'static Client {
    static HTTP_CLIENT: OnceLock<Client> = OnceLock::new();
    HTTP_CLIENT.get_or_init(|| Client::new())
}

pub async fn login(email: String, password: String) -> Result<LoginResponse, StatusCode> {
    let body = LoginRequest{email, password};
    let client = http_client();
    let res = client.post(BASE_URL.to_owned() + "/user/login")
        .json(&body)
        .send()
        .await
        .expect("Error while sending login request");
    if res.status() == 200 {
        Ok(res.json::<LoginResponse>().await.unwrap())
    } else {
        Err(res.status())
    }
}
