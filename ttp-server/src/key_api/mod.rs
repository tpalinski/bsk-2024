use leptos::{server_fn::codec::Json, *};
use serde::{Deserialize, Serialize};

use crate::{api_utils::validate_user_token, user_repository::{get_private_key, get_public_key, replace_token}};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetPublicKeyResponse {
    pub key: String, 
}

#[server(name=GetPublicKeyRequest, prefix="/api", endpoint="pubkey", input = Json, output = Json)]
async fn pubkey(email: String) -> Result<GetPublicKeyResponse, ServerFnError> {
    let key = get_public_key(email).await;
    match key {
        Ok(key) => Ok(GetPublicKeyResponse { key }),
        Err(e) => match e {
            _ => {
                let resp = expect_context::<leptos_actix::ResponseOptions>();
                resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
                Err(ServerFnError::ServerError("No user found".to_owned()))
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetPrivateKeyResponse {
    pub key: String, 
    pub token: String
}

#[server(name=GetPrivateKeyRequest, prefix="/api", endpoint="privkey", input = Json, output = Json)]
async fn privkey(email: String, token: String) -> Result<GetPrivateKeyResponse, ServerFnError> {
    let _ = validate_user_token(token, email.clone()).await?;
    let key = match get_private_key(email.clone()).await {
        Ok(key) => key,
        Err(e) => match e {
            _ => {
                let resp = expect_context::<leptos_actix::ResponseOptions>();
                resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
                return Err(ServerFnError::ServerError("No user found".to_owned()))
            }
        }
    };
    let new_token = replace_token(email).await?;
    Ok(GetPrivateKeyResponse{key, token: new_token})
}
