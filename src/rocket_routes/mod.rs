use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{json, Json, Value};
use std::error::Error;
use rocket::request::{FromRequest, Outcome, Request };
use crate::models;
use rocket_db_pools::Connection;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use crate::models::{RoleCode, User};
use crate::repositories::{RoleRepository, UserRepository};

pub mod crates;
pub mod rustaceans;
pub mod authorization;

#[derive(rocket_db_pools::Database)]
#[database("pg_db")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);

#[derive(rocket_db_pools::Database)]
#[database("redis")]
pub struct CacheConn(rocket_db_pools::deadpool_redis::Pool);

pub fn server_error(e: Box<dyn Error>) -> Custom<Value> {
    rocket::error!("{}", e);
    Custom(Status::InternalServerError, json!("Error"))
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for models::User {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // check Authorization header
        let session_header = req.headers()
            .get_one("Authorization")
            .map(|h| h.split_whitespace().collect::<Vec<_>>())
            .filter(|v| v.len() == 2 && v[0] == "Bearer");

        if let Some(header_value) = session_header {
            // TODO: use -> try_outcome!()
            let mut cache = req.guard::<Connection<CacheConn>>().await
                .expect("Cache connection");
            let result = cache.get::<String, i32>(format!("sessions/{}", header_value[1])).await;
            if let Ok(user_id) = result {
                let mut db = req.guard::<Connection<DbConn>>().await
                    .expect("DB connection");
                if let Ok(user) = UserRepository::find(&mut db, user_id).await {
                    return Outcome::Success(user);
                }
            }
        }
        Outcome::Error((Status::Unauthorized,()))
    }
}

pub struct EditorUser(User);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for EditorUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = request.guard::<User>().await.expect("Cannot retrieve current logged user");
        let mut db = request.guard::<Connection<DbConn>>().await.expect("Cannot retrieve DB connection");
        let roles = RoleRepository::find_by_user(&mut db, &user).await.expect("Cannot retrieve user roles");
        let is_editor = roles.iter().any(|r| match r.code {
            RoleCode::Editor => true,
            RoleCode::Admin => true,
            _ => false
        });
        rocket::info!("User {:?} is editor: {}", user, is_editor);
        if is_editor {
            return Outcome::Success(EditorUser(user));
        };
        Outcome::Error((Status::Forbidden, ()))
    }

}
