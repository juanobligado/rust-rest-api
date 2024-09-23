use reqwest::blocking::{Client, Response};
use reqwest::StatusCode;
use rocket::serde::json::json;
use rocket::serde::json::Value;

fn create_test_rustacean(client: &Client) -> Value {
    let response = client.post("http://127.0.0.1:8000/rustaceans")
        .json(&json!({
            "name": "John Doe",
            "email": "foo@bar"
        }))
        .send().unwrap();
    response.json().unwrap()
}

#[test]
fn test_get_rustaceans(){
    // Test get_rustaceans
    // GET /rustaceans
    let client = Client::new();
    let rustacean1 = create_test_rustacean(&client);
    let rustacean2 = create_test_rustacean(&client);
    let response = client.get("http://127.0.0.1:8000/rustaceans").send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&rustacean1));
    assert!(json.as_array().unwrap().contains(&rustacean2));
}

#[test]
fn test_create_rustacean(){
    let client = Client::new();
    let response = client.post("http://127.0.0.1:8000/rustaceans")
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
}

#[test]
fn test_view_rustacean(){
    let client = Client::new();
    let rustacean  = create_test_rustacean(&client);
    let response = client.get(format!("http://127.0.0.1:8000/rustaceans/{}", rustacean["id"]))
        .send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let rustacean: Value = response.json().unwrap();
    assert_eq!(rustacean, json!({
        "id": rustacean["id"],
        "name": "John Doe",
        "email": "foo@bar",
        "created_at": rustacean["created_at"]
    }));
}

#[test]
fn test_update_rustacean(){
    let client = Client::new();
    let rustacean  = create_test_rustacean(&client);
    let response = client.put(format!("http://127.0.0.1:8000/rustaceans/{}", rustacean["id"]))
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
}

#[test]
fn test_delete_rustacean(){
    let client = Client::new();
    let rustacean  = create_test_rustacean(&client);
    let response = client.delete(format!("http://127.0.0.1:8000/rustaceans/{}", rustacean["id"]))
        .send().unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

