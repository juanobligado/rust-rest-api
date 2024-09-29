use diesel_async::{AsyncPgConnection, AsyncConnection};
use crate::auth::hash_password;
use crate::models::{NewUser, RoleCode};
use crate::repositories::{RoleRepository, UserRepository};
use std::str::FromStr;

async fn load_db_connection() -> AsyncPgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    AsyncPgConnection::establish(&database_url).await
        .expect(&format!("Error connecting to {}", database_url))
}
pub async fn create_user(username: &String, password: &String, role_codes: Vec<String>) {
    let mut c = load_db_connection().await;
    let password_hash = hash_password(password).unwrap();
    let new_user = NewUser { username: username.to_owned(), password: password_hash.to_string() };
    let roles = role_codes.iter().map(|r| RoleCode::from_str(r.as_str()).unwrap()).collect();
    let user = UserRepository::create(&mut c, new_user, roles).await.unwrap();
    println!("Created user: {:?}", user);
    let roles = RoleRepository::find_by_user(&mut c, &user).await.unwrap();
    println!("User roles: {:?}", roles);
}

pub async fn delete_user(id: i32) {
    let mut c=load_db_connection().await;
    let deleted_count = UserRepository::delete(&mut c, id).await.unwrap();
    println!("Deleted {} users", deleted_count);
}

pub async fn list_users() {
    let mut c = load_db_connection().await;
    let users = UserRepository::find_with_roles(&mut c).await.unwrap();
    for user in users {
        println!("{:?}", user);
    }
}