use crate::models::{Crate, NewCrate, NewRustacean, Rustacean};
use crate::repositories::{CrateRepository, RustaceanRepository};
use crate::rocket_routes::{server_error, DbConn};
use rocket::http::Status;
use rocket::log::private::error;
use rocket::response::status::{Custom, NoContent};
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;
use std::any::Any;

#[rocket::get("/crates")]
pub async fn get_crates(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    CrateRepository::find_multiple(&mut db, 100)
        .await
        .map(|r| json!(r))
        .map_err(|e| server_error(e.into()))
}

#[rocket::get("/crates/<id>")]
pub async fn get_crate_from_id(
    mut db: Connection<DbConn>,
    id: i32,
) -> Result<Value, Custom<Value>> {
    CrateRepository::find(&mut db, id)
        .await
        .map(|r| json!(r))
        .map_err(|e| match e {
            diesel::NotFound => Custom(Status::NotFound, json!({"error": "Crate not found"})),
            _ => server_error(e.into()),
        })
}

#[rocket::post("/crates", format = "json", data = "<new_crate>")]
pub async fn create_crate(
    mut db: Connection<DbConn>,
    new_crate: Json<NewCrate>,
) -> Result<Custom<Value>, Custom<Value>> {
    CrateRepository::create(&mut db, new_crate.into_inner())
        .await
        .map(|r| Custom(Status::Created, rocket::serde::json::serde_json::json!(r)))
        .map_err(|e| server_error(e.into()))
}

#[rocket::put("/crates/<id>", format = "json", data = "<a_crate>")]
pub async fn update_crate(
    mut db: Connection<DbConn>,
    a_crate: Json<Crate>,
    id: i32,
) -> Result<Value, Custom<Value>> {
    CrateRepository::update(&mut db, id, a_crate.into_inner())
        .await
        .map(|r| rocket::serde::json::serde_json::json!(r))
        .map_err(|e| server_error(e.into()))
}

#[rocket::delete("/crates/<id>")]
pub async fn delete_crate(mut db: Connection<DbConn>, id: i32) -> Result<NoContent, Custom<Value>> {
    CrateRepository::delete(&mut db, id)
        .await
        .map(|_| NoContent)
        .map_err(|e| server_error(e.into()))
}
