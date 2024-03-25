use base64ct::{Base64, Encoding};
use leptos::{server_fn::codec::Json, *};
use sha2::{Digest, Sha256};

use crate::user_repository::{create_user, db_models::User, user_exists, utils::generate_token};

#[server(name=RegisterUserRequest, prefix="/api/user", endpoint="register", input = Json, output = Json)]
pub async fn register_user(email: String, password: String) -> Result<(), ServerFnError> {
    let user_exists = match user_exists(email.clone()).await {
        Ok(r) => r,
        Err(_) => {
            let resp = expect_context::<leptos_actix::ResponseOptions>();
            resp.set_status(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
            return Err(ServerFnError::ServerError("Error while checking DB".to_owned()))
        }
    };
    if user_exists {
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::CONFLICT);
        return Err(ServerFnError::ServerError("User already exists".to_owned()))
    }

    let jwt = generate_token(email.clone());
    let hashed_password = Sha256::digest(password).to_vec();
    let hashed_password = Base64::encode_string(&hashed_password);
    let user = User::new(hashed_password, jwt);
    match create_user(user, email).await {
        Ok(_) => Ok(()),
        Err(e) => {
            dbg!(e);
            Ok(())
        }
    }
}
