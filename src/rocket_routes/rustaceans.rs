use rocket_db_pools::Connection;
use rocket::serde::json::{Json, Value, json};
use rocket::response::status::{Custom, NoContent};
use crate::repositories::RustaceanRepository;
use crate::rocket_routes::{server_error, DbConn};
use rocket::http::Status;
use rocket::log::private::error;
use rocket_db_pools::Database;
use crate::models::*;

#[rocket::get("/rustaceans")]
pub async fn get_rustaceans(mut db: Connection<DbConn>) -> Result<Value,Custom<Value>> {
    RustaceanRepository::find_multiple(& mut db,100).await
        .map(|r| json!(r) )
        .map_err(|e| {
            error!("{}",e);
            Custom(Status::InternalServerError, json!("Error"))
        } )
}

#[rocket::get("/rustaceans/<id>")]
pub async fn get_rustacean_from_id(mut db: Connection<DbConn>, id: i32) -> Result<Value,Custom<Value>> {
    RustaceanRepository::find(& mut db, id).await
        .map(|r| json!(r) )
        .map_err(|e| match e {
            diesel::NotFound => Custom(Status::NotFound, json!({"error": "Rustacean not found"})),
            _ => server_error(e.into())
        })
}

#[rocket::post("/rustaceans", format="json" ,data = "<new_rustacean>")]
pub async fn create_rustacean(mut db: Connection<DbConn>, new_rustacean: Json<NewRustacean>) -> Result<Custom<Value>,Custom<Value>> {
    RustaceanRepository::create(& mut db, new_rustacean.into_inner()).await
        .map(|r| Custom(Status::Created ,json!(r)) )
        .map_err(|e| server_error(e.into()) )
}

#[rocket::put("/rustaceans/<id>", format="json" ,data = "<rustacean>")]
pub async fn update_rustacean(mut db: Connection<DbConn>, rustacean: Json<Rustacean>, id: i32) -> Result<Value,Custom<Value>> {
    RustaceanRepository::update(& mut db, id, rustacean.into_inner()).await
        .map(|r| json!(r) )
        .map_err(|e| server_error(e.into()) )
}

#[rocket::delete("/rustaceans/<id>")]
pub async fn delete_rustacean(mut db: Connection<DbConn>, id: i32) -> Result<NoContent,Custom<Value>> {
    RustaceanRepository::delete(& mut db, id).await
        .map(|_| NoContent )
        .map_err(|e| server_error(e.into()) )
}