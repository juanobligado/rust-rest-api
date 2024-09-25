use reqwest::blocking::Client;
use serde_json::{json, Value};

pub static APP_HOST: &'static str = "http://127.0.0.1:8000";
pub fn delete_rustacean(client: &Client, rustacean: Value) {
    client.delete(format!("{}/rustaceans/{}", APP_HOST, rustacean["id"]))
        .send().unwrap();
}

pub fn delete_crate(client: &Client, _crate: Value) {
    client.delete(format!("{}/crates/{}", APP_HOST, _crate["id"]))
        .send().unwrap();
}
pub fn create_test_rustacean(client: &Client) -> Value {
    let response = client.post(format!("{}/rustaceans", APP_HOST))
        .json(&json!({
            "name": "John Doe",
            "email": "foo@bar"
        }))
        .send().unwrap();
    response.json().unwrap()
}

pub fn create_test_crate(client: &Client, rustacean: &Value) -> Value {
    let response = client.post(format!("{}/crates", APP_HOST))
        .json(&json!({
            "name": "test crate",
            "version": "0.1.0",
            "code": "test_crate",
            "description": "A test crate",
            "rustacean_id": rustacean["id"]
        }))
        .send().unwrap();
    response.json().unwrap()
}
