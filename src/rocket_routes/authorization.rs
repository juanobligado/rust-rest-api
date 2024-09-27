use argon2::{PasswordHash, PasswordVerifier};
use rocket::response::status::Custom;
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;
use tokio::io::join;
use crate::auth::{ authorize_user, Credentials};
use crate::repositories::UserRepository;
use crate::rocket_routes::{DbConn, server_error, CacheConn};
#[rocket::post("/login", format = "json", data = "<credentials>")]
pub async fn login(mut db: Connection<DbConn>, mut cache: Connection<CacheConn>, credentials: Json<Credentials>) -> Result<Value,Custom<Value>> {
    UserRepository::find_by_username(&mut db, &credentials.username).await
        .map(|user| {
            if let Ok(token) = authorize_user(user, credentials.into_inner()) {
                return json!(token);
            }
            json!("Invalid credentials")
        })
        .map_err(|e| {
            server_error(e.into())
        })

}