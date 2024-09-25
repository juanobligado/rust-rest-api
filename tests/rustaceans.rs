use reqwest::blocking::{Client, Response};
use reqwest::StatusCode;
use serde_json::{Value, json};

pub mod common;
use common::{delete_rustacean, create_test_rustacean, APP_HOST};

#[test]
fn test_get_rustaceans(){
    // Test get_rustaceans
    // GET /rustaceans
    let client = Client::new();
    let rustacean1 = common::create_test_rustacean(&client);
    let rustacean2 = common::create_test_rustacean(&client);
    let response = client.get(format!("{}/rustaceans",APP_HOST)).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&rustacean1));
    assert!(json.as_array().unwrap().contains(&rustacean2));
    delete_rustacean(&client, rustacean1);
    delete_rustacean(&client, rustacean2);
}

#[test]
fn test_create_rustacean(){
    let client = Client::new();
    let response = client.post(format!("{}/rustaceans",APP_HOST))
        .json(&json!({
            "name": "John Doe",
            "email": "foo@bar"
        }))
        .send().unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::CREATED);
    let rustacean: Value = response.json().unwrap();
    assert_eq!(rustacean, json!({
        "id": rustacean["id"],
        "name": "John Doe",
        "email": "foo@bar",
        "created_at": rustacean["created_at"]
    }));
    delete_rustacean(&client, rustacean);
}

#[test]
fn test_view_rustacean(){
    let client = Client::new();
    let rustacean  = create_test_rustacean(&client);
    let response = client.get(format!("{}/rustaceans/{}", APP_HOST, rustacean["id"]))
        .send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let rustacean: Value = response.json().unwrap();
    assert_eq!(rustacean, json!({
        "id": rustacean["id"],
        "name": "John Doe",
        "email": "foo@bar",
        "created_at": rustacean["created_at"]
    }));
    // View invalid rustacean
    let invalid_rustacean_id = 9999;
    let response = client.get(format!("{}/rustaceans/{}", APP_HOST, invalid_rustacean_id))
        .send().unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    delete_rustacean(&client, rustacean);
}

#[test]
fn test_update_rustacean(){
    let client = Client::new();
    let rustacean  = create_test_rustacean(&client);
    let response = client.put(format!("{}/rustaceans/{}", APP_HOST, rustacean["id"]))
        .json(&json!({
            "id": rustacean["id"],
            "name": "Jane Doe",
            "email": "bar@foo",
            "created_at": rustacean["created_at"]
        }))
        .send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let rustacean: Value = response.json().unwrap();
    assert_eq!(rustacean, json!({
        "id": rustacean["id"],
        "name": "Jane Doe",
        "email": "bar@foo",
        "created_at": rustacean["created_at"]
    }));
    delete_rustacean(&client, rustacean);
}

#[test]
fn test_delete_rustacean(){
    let client = Client::new();
    let rustacean  = create_test_rustacean(&client);
    let response = client.delete(format!("{}/rustaceans/{}", APP_HOST, rustacean["id"]))
        .send().unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

