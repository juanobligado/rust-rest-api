use std::error::Error;
use rocket::http::Status;
use rocket::response::status::{Custom};
use rocket::serde::json::{ Value, Json, json };
use rocket_db_pools::Database;

pub mod rustaceans;
pub mod crates;


#[derive(Database)]
#[database("pg_db")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);

pub fn server_error(e: Box<dyn Error>) -> Custom<Value> {
    rocket::error!("{}", e);
    Custom(Status::InternalServerError, json!("Error"))
}