use std::process::Command;
use reqwest::blocking::{Client, ClientBuilder};
use reqwest::header;
use serde_json::{json, Value};

pub static APP_HOST: &'static str = "http://127.0.0.1:8000";
pub fn delete_rustacean(client: &Client, rustacean: Value) {
    client
        .delete(format!("{}/rustaceans/{}", APP_HOST, rustacean["id"]))
        .send()
        .unwrap();
}

pub fn delete_crate(client: &Client, _crate: Value) {
    client
        .delete(format!("{}/crates/{}", APP_HOST, _crate["id"]))
        .send()
        .unwrap();
}
pub fn create_test_rustacean(client: &Client) -> Value {
    let response = client
        .post(format!("{}/rustaceans", APP_HOST))
        .json(&json!({
            "name": "John Doe",
            "email": "foo@bar"
        }))
        .send()
        .unwrap();
    response.json().unwrap()
}

pub fn create_test_crate(client: &Client, rustacean: &Value) -> Value {
    let response = client
        .post(format!("{}/crates", APP_HOST))
        .json(&json!({
            "name": "test crate",
            "version": "0.1.0",
            "code": "test_crate",
            "description": "A test crate",
            "rustacean_id": rustacean["id"]
        }))
        .send()
        .unwrap();
    response.json().unwrap()
}

pub fn get_client_with_logged_in_admin() -> Client {
    Command::new("cargo")
        .args(&["run", "--bin", "cli", "users", "create", "test", "1234", "admin"])
        .output().unwrap();
    let client = Client::new();
    let response = client
        .post(format!("{}/login", APP_HOST))
        .json(&json!({
            "username": "test",
            "password": "1234"
        }))
        .send()
        .unwrap();
    let json: Value = response.json().unwrap();
    assert!(json.get("token").is_some());
    assert_eq!(json["token"].as_str().unwrap().len(),128);
    let header_value = format!("Bearer {}", json["token"].as_str().unwrap());
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&header_value).unwrap(),
    );
    ClientBuilder::new().default_headers(headers).build().unwrap()
}

pub fn get_client_with_logged_in_viewer() -> Client {
    Command::new("cargo")
        .args(&["run", "--bin", "cli", "users", "create", "test_viewer", "1234", "viewer"])
        .output().unwrap();
    let client = Client::new();
    let response = client
        .post(format!("{}/login", APP_HOST))
        .json(&json!({
            "username": "test_viewer",
            "password": "1234"
        }))
        .send()
        .unwrap();
    let json: Value = response.json().unwrap();
    assert!(json.get("token").is_some());
    assert_eq!(json["token"].as_str().unwrap().len(),128);
    let header_value = format!("Bearer {}", json["token"].as_str().unwrap());
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&header_value).unwrap(),
    );
    ClientBuilder::new().default_headers(headers).build().unwrap()
}

pub fn get_client_with_not_logged_admin() -> Client {
    let headers = header::HeaderMap::new();
    ClientBuilder::new().default_headers(headers).build().unwrap()
}