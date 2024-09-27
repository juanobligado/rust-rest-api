use rocket_db_pools::Database;
extern crate backend_crate;
#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            rocket::routes![
                backend_crate::rocket_routes::authorization::login,
                backend_crate::rocket_routes::rustaceans::get_rustaceans,
                backend_crate::rocket_routes::rustaceans::get_rustacean_from_id,
                backend_crate::rocket_routes::rustaceans::create_rustacean,
                backend_crate::rocket_routes::rustaceans::update_rustacean,
                backend_crate::rocket_routes::rustaceans::delete_rustacean,
                backend_crate::rocket_routes::crates::get_crates,
                backend_crate::rocket_routes::crates::get_crate_from_id,
                backend_crate::rocket_routes::crates::create_crate,
                backend_crate::rocket_routes::crates::update_crate,
                backend_crate::rocket_routes::crates::delete_crate,
            ],
        )
        .attach(backend_crate::rocket_routes::DbConn::init())
        .attach(backend_crate::rocket_routes::CacheConn::init())
        .launch()
        .await;
}
