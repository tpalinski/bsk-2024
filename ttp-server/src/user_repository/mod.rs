use std::any::Any;

use once_cell::sync::Lazy;
use serde::Deserialize;
use surrealdb::{engine::remote::ws::{Client, Ws}, sql::Thing, Surreal};

use crate::model::user_keys::UserKeys;

use self::db_models::UserKeyData;

static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

pub mod db_models;

#[derive(Debug, Clone)]
pub enum DBError {
    NO_RECORD
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
    let _created: Vec<Record> = DB.create(("user", &email))
        .content(record_data)
        .await?
        .expect("DB error");
    Ok(())
}

pub async fn get_public_key(email: String) -> Result<String, DBError> {
    let user: Option<UserKeyData> = DB.select(("user", &email)).await.expect("DBconnection Error");
    match user {
        Some(res) => Ok(res.pubkey()),
        None => Err(DBError::NO_RECORD)
    }
}
