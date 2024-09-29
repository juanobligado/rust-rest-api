use reqwest::StatusCode;
use serde_json::{json, Value};
pub mod common;
use common::*;
#[test]
fn test_create_crate() {
    let client = get_client_with_logged_in_admin();
    let rustacean = create_test_rustacean(&client);
    let response = client
        .post(format!("{}/crates", APP_HOST))
        .json(&json!({
            "name": "my_crate",
            "version": "0.1.0",
            "code": "mycrate",
            "description": "A test crate",
            "rustacean_id": rustacean["id"]
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::CREATED);
    let crate_: Value = response.json().unwrap();
    assert_eq!(
        crate_,
        json!({
            "id": crate_["id"],
            "name": "my_crate",
            "version": "0.1.0",
            "code": "mycrate",
            "description": "A test crate",
            "rustacean_id": rustacean["id"],
            "created_at": crate_["created_at"]
        })
    );
    delete_crate(&client, crate_);
    delete_rustacean(&client, rustacean);
}

#[test]
fn test_view_crate() {
    let client = get_client_with_logged_in_admin();
    let rustacean = create_test_rustacean(&client);
    let test_crate = create_test_crate(&client, &rustacean);
    let response = client
        .get(format!("{}/crates/{}", APP_HOST, test_crate["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let test_crate_view: Value = response.json().unwrap();
    assert_eq!(
        test_crate_view,
        json!({
            "id": test_crate_view["id"],
            "rustacean_id": rustacean["id"],
            "name": "test crate",
            "version": "0.1.0",
            "code": "test_crate",
            "description": "A test crate",
            "created_at": test_crate_view["created_at"]
        })
    );
    // View invalid crate
    let invalid_crate_id = 9999;
    let response = client
        .get(format!("{}/crates/{}", APP_HOST, invalid_crate_id))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    delete_crate(&client, test_crate);
    delete_rustacean(&client, rustacean);
}


#[test]
fn test_update_crate() {
    let client = get_client_with_logged_in_admin();
    let rustacean = create_test_rustacean(&client);
    let test_crate = create_test_crate(&client, &rustacean);
    let response = client
        .put(format!("{}/crates/{}", APP_HOST, test_crate["id"]))
        .json(&json!({
            "id": test_crate["id"],
            "name": "Other Test crate",
            "code": test_crate["code"],
            "description": test_crate["description"],
            "version": test_crate["version"],
            "rustacean_id": rustacean["id"],
            "created_at": test_crate["created_at"]
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let updated_crate: Value = response.json().unwrap();
    assert_eq!(
        updated_crate,
        json!({
                "id": test_crate["id"],
                "name": "Other Test crate",
                "code": test_crate["code"],
                "version": test_crate["version"],
                "description": test_crate["description"],
                "rustacean_id": updated_crate["rustacean_id"],
                "created_at": test_crate["created_at"]
        })
    );

    //Should fail to update the crate
    let response = client
        .put(format!("{}/crates/{}", APP_HOST, test_crate["id"]))
        .json(&json!({
            "id": test_crate["id"],
            "name": "Other Test crate",
            "code": test_crate["code"],
            "description": test_crate["description"],
            "version": test_crate["version"],
            "rustacean_id": 9999,
            "created_at": test_crate["created_at"]
        }))
        .send()
        .unwrap();
    assert_eq!(
        response.status(),
        reqwest::StatusCode::INTERNAL_SERVER_ERROR
    );

    delete_crate(&client, test_crate);
    delete_rustacean(&client, updated_crate);
}

#[test]
fn test_delete_crate() {
    let client = get_client_with_logged_in_admin();
    let rustacean = create_test_rustacean(&client);
    let test_crate = create_test_crate(&client, &rustacean);
    let response = client
        .delete(format!("{}/crates/{}", APP_HOST, test_crate["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    delete_rustacean(&client, rustacean);
}

#[test]
fn test_list_crates() {
    let client = get_client_with_logged_in_admin();
    let rustacean = create_test_rustacean(&client);
    let test_crate = create_test_crate(&client, &rustacean);
    let other_crate = create_test_crate(&client, &rustacean);

    let response = client.get(format!("{}/crates", APP_HOST)).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let crates: Vec<Value> = response.json().unwrap();
    assert!(crates.contains(&test_crate));
    assert!(crates.contains(&other_crate));
    delete_crate(&client, test_crate);
    delete_crate(&client, other_crate);
    delete_rustacean(&client, rustacean);
}
