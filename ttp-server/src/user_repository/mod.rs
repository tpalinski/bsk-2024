use once_cell::sync::Lazy;
use serde::Deserialize;
use surrealdb::{engine::remote::ws::{Client, Ws}, opt::PatchOp, sql::Thing, Surreal};

use crate::model::user_keys::UserKeys;

use self::{db_models::{User, UserKeyData}, utils::generate_token};

static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

pub mod db_models;
pub mod utils;

#[derive(Debug, Clone)]
pub enum DBError {
    NO_RECORD,
    USER_EXISTS,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}


/*
*  Initialize connection to surrealdb database
*/
pub async fn db() -> surrealdb::Result<()> {   
    DB.connect::<Ws>("localhost:8000").await?;
    DB.use_ns("bsk").use_db("user_keys").await?;
    Ok(())
}

pub async fn insert_keys(email: String, keys: &UserKeys) -> surrealdb::Result<()> {
    let record_data: UserKeyData = keys.to_owned().into(); 
    let _created: Vec<Record> = DB.create(("keys", &email))
        .content(record_data)
        .await?
        .expect("DB error");
    Ok(())
}

pub async fn user_exists(email: String) -> Result<bool, DBError> {
    let user: Option<User> = DB.select(("user", &email)).await.expect("DBconnection Error");
    Ok(user.is_some())
}

pub async fn create_user(user: User, email: String) -> surrealdb::Result<()> {
    let _: Record = DB.create(("user", &email))
        .content(user)
        .await?
        .expect("DB error");
    Ok(())
}

pub async fn get_user(email: String) -> surrealdb::Result<Option<User>> {
    let user = DB.select(("user", email)).await;
    user
}

pub async fn validate_token(email: String, token: String) -> Result<bool, DBError> {
    let user: Option<User> = DB.select(("user", &email)).await.expect("DBconnection Error");
    match user {
        Some(user) => Ok(user.token == token),
        None => Err(DBError::NO_RECORD)
    }
}

pub async fn replace_token(email: String) -> surrealdb::Result<String> {
    let token = generate_token(email.clone());
    let _: Option<User> = DB.update(("user", &email))
        .patch(PatchOp::replace("/token", &token))
        .await?;
    Ok(token)
}

pub async fn get_public_key(email: String) -> Result<String, DBError> {
    let user: Option<UserKeyData> = DB.select(("keys", &email)).await.expect("DBconnection Error");
    match user {
        Some(res) => Ok(res.public_key),
        None => Err(DBError::NO_RECORD)
    }
}

pub async fn get_private_key(email: String) -> Result<String, DBError> {
    let user: Option<UserKeyData> = DB.select(("keys", &email)).await.expect("DBconnection Error");
    match user {
        Some(res) => Ok(res.private_key),
        None => Err(DBError::NO_RECORD)
    }
}
