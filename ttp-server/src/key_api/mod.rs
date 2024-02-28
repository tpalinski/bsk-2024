use leptos::{server_fn::codec::Json, *};

use crate::user_repository::get_public_key;



#[server(name=GetPublicKeyRequest, prefix="/api", endpoint="pubkey", input = Json, output = Json)]
async fn pubkey(email: String) -> Result<String, ServerFnError> {
    let key = get_public_key(email).await;
    match key {
        Ok(key) => Ok(key),
        Err(e) => match e {
            crate::user_repository::DBError::NO_RECORD => {
                let resp = expect_context::<leptos_actix::ResponseOptions>();
                resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
                Err(ServerFnError::WrappedServerError(server_fn::error::NoCustomError))
            }
        }
    }
}
