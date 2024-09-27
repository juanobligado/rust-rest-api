use argon2::{PasswordHash, PasswordVerifier};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use tokio::io::join;
use crate::auth::{ authorize_user, Credentials};
use crate::repositories::UserRepository;
use crate::rocket_routes::{DbConn, server_error, CacheConn};
#[rocket::post("/login", format = "json", data = "<credentials>")]
pub async fn login(mut db: Connection<DbConn>, mut cache: Connection<CacheConn>, credentials: Json<Credentials>) -> Result<Value,Custom<Value>> {
    let user= UserRepository::find_by_username(&mut db, &credentials.username).await
        .map_err(|e| {
            match e {
                diesel::NotFound => Custom(Status::Unauthorized, json!("Missing User")) ,
                _ => server_error(e.into())
            }
        })?;
    let session_id = authorize_user(&user, credentials.into_inner())
        .map_err(|e|
            Custom(Status::Unauthorized, json!("Wrong credentials"))
        )?;
    cache.set_ex::<String,i32,()>(
        format!("sessions/{}",session_id),
        user.id,
        3600
    ).await
    .map_err(|e| { server_error(e.into()) })?;
    Ok(json!({"token": session_id}))
}