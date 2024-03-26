use leptos::*;

use crate::user_repository::{user_exists, validate_token};

pub async fn validate_user_token(token: String, email: String) -> Result<(), ServerFnError> {
    match validate_token(email.clone(), token.clone()).await {
        Ok(res) => {
            if !res {
                let resp = expect_context::<leptos_actix::ResponseOptions>();
                resp.set_status(actix_web::http::StatusCode::FORBIDDEN);
                return Err(ServerFnError::ServerError("Error while authenticating user. Are you sure you are logged in?".to_owned()))
            }
        },
        Err(_) => {
            return Err(ServerFnError::ServerError("Tokens do not match".to_owned()))
        }
    };
    Ok(())
}

pub async fn check_user(email: String) -> Result<bool, ServerFnError> {
    let user_exists = match user_exists(email.clone()).await {
        Ok(r) => r,
        Err(_) => {
            let resp = expect_context::<leptos_actix::ResponseOptions>();
            resp.set_status(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
            return Err(ServerFnError::ServerError("Error while checking DB".to_owned()))
        }
    };
    Ok(user_exists)
}
