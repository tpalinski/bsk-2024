use leptos::{server_fn::codec::Json, *};

use crate::user_repository::get_public_key;



#[server(name=GetPublicKeyRequest, prefix="/api", endpoint="pubkey", input = Json, output = Json)]
async fn pubkey(email: String) -> Result<String, ServerFnError> {
    let key = get_public_key(email).await;
    match key {
        Ok(key) => Ok(key),
        Err(e) => match e {
            crate::user_repository::DBError::NO_RECORD => Err(ServerFnError::ServerError("No user found".to_owned()))
        }
    }
}
