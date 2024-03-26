use leptos::{server_fn::codec::Json, *};
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenResponse {
    pub token: String
}

impl TokenResponse {
    pub fn new(token: String) -> Self {
        TokenResponse{token}
    }
}

#[server(name=RegisterUserRequest, prefix="/api/user", endpoint="register", input = Json, output = Json)]
pub async fn register_user(email: String, password: String, user_name: String) -> Result<TokenResponse, ServerFnError> {
    #[cfg(feature="ssr")]
    {
        use base64ct::{Base64, Encoding};
        use sha2::{Digest, Sha256};
        use crate::api_utils::check_user;
        use crate::user_repository::{create_user, db_models::User, utils::generate_token};


        let user_exists = check_user(email.clone()).await?;
        if user_exists {
            let resp = expect_context::<leptos_actix::ResponseOptions>();
            resp.set_status(actix_web::http::StatusCode::CONFLICT);
            return Err(ServerFnError::ServerError("User already exists".to_owned()))
        }

        let jwt = generate_token(email.clone());
        let hashed_password = Sha256::digest(password).to_vec();
        let hashed_password = Base64::encode_string(&hashed_password);
        let user = User::new(hashed_password, jwt.clone(), user_name);
        match create_user(user, email).await {
            Ok(_) => Ok(TokenResponse::new(jwt)),
            Err(e) => {
                dbg!(e);
                Err(ServerFnError::ServerError("Error while creating user".to_owned()))
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserInfoResponse {
    pub email: String,
    pub token: String,
    pub name: String
}

#[server(name=LoginUserRequest, prefix="/api/user", endpoint="login", input = Json, output = Json)]
pub async fn login_user(email: String, password: String) -> Result<UserInfoResponse, ServerFnError> {
    #[cfg(feature="ssr")]
    {
        use crate::api_utils::check_user;
        use crate::user_repository::replace_token;
        use crate::user_repository::{get_user, utils::check_passwords};

        let user_exists = check_user(email.clone()).await?;
        if !user_exists {
            let resp = expect_context::<leptos_actix::ResponseOptions>();
            resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
            return Err(ServerFnError::ServerError("User does not exist".to_owned()))
        }
        let user = get_user(email.clone()).await?.unwrap(); // This should never throw, if id does just 500 it
        if !check_passwords(password, user.password) {
            let resp = expect_context::<leptos_actix::ResponseOptions>();
            resp.set_status(actix_web::http::StatusCode::UNAUTHORIZED);
            return Err(ServerFnError::ServerError("Wrong password provided".to_owned()))
        }
        let token = replace_token(email.clone()).await?;
        Ok(UserInfoResponse{token, email, name: user.name})
    }
}
