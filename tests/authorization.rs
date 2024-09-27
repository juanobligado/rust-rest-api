use std::process::Command;
use serde_json::{json, Value};
use reqwest::blocking::{Client};
use reqwest::StatusCode;
pub mod common;
use common::{APP_HOST};
#[test]
fn test_login() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "cli", "users", "create", "test", "1234", "admin"])
        .output().unwrap();
    println!("{:?}",output);
    let client = Client::new();
    let response = client
        .post(format!("{}/login", APP_HOST))
        .json(&json!({
            "username": "test",
            "password": "1234"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.get("token").is_some());
    assert_eq!(json["token"].as_str().unwrap().len(),128);

    let response = client
        .post(format!("{}/login", APP_HOST))
        .json(&json!({
            "username": "test",
            "password": "34"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

}