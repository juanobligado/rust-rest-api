use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket::serde::json::{ Value, Json, json };
use rocket_db_pools::Connection;
use crate::DbConn;
use crate::models::{Crate, NewCrate, NewRustacean, Rustacean};
use crate::repositories::{CrateRepository, RustaceanRepository};

#[rocket::get("/crates")]
pub async fn get_crates(mut db: Connection<DbConn>) -> Result<Value,Custom<Value>> {
    CrateRepository::find_multiple(& mut db,100).await
        .map(|r| json!(r) )
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")) )
}

#[rocket::get("/crates/<id>")]
pub async fn get_crate_from_id(mut db: Connection<DbConn>, id: i32) -> Result<Value,Custom<Value>> {
    CrateRepository::find(& mut db, id).await
        .map(|r| json!(r) )
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")) )
}

#[rocket::post("/crates", format="json" ,data = "<new_crate>")]
pub async fn create_crate(mut db: Connection<DbConn>, new_crate: Json<NewCrate>) -> Result<Custom<Value>,Custom<Value>> {
    CrateRepository::create(& mut db, new_crate.into_inner()).await
        .map(|r| Custom(Status::Created, rocket::serde::json::serde_json::json!(r)) )
        .map_err(|_| Custom(Status::InternalServerError, rocket::serde::json::serde_json::json!("Error")) )
}

#[rocket::put("/crates/<id>", format="json" ,data = "<a_crate>")]
pub async fn update_crate(mut db: Connection<DbConn>, a_crate: Json<Crate>, id: i32) -> Result<Value,Custom<Value>> {
    CrateRepository::update(& mut db, id, a_crate.into_inner()).await
        .map(|r| rocket::serde::json::serde_json::json!(r) )
        .map_err(|_| Custom(Status::InternalServerError, rocket::serde::json::serde_json::json!("Error")) )
}

#[rocket::delete("/crates/<id>")]
pub async fn delete_crate(mut db: Connection<DbConn>, id: i32) -> Result<NoContent,Custom<Value>> {
    CrateRepository::delete(& mut db, id).await
        .map(|_| NoContent )
        .map_err(|_| Custom(Status::InternalServerError, rocket::serde::json::serde_json::json!("Error")) )
}