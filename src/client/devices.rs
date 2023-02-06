use crate::commands::configuration::{Configuration};
use crate::client::output::device_view::{view_device, view_list};
use reqwest::{Client, StatusCode};
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use crate::client::output::routes_view::view_routes;

static LIST_DEVICES: &str = "/api/v2/tailnet/";
static DEVICE: &str = "/api/v2/device/";



pub async fn list_devices(scope: String) {
    let auth = Configuration::get_config().unwrap();
    let request_string = format!("{}{}{}/devices", auth.api_uri, LIST_DEVICES, scope);
    let curl = Client::new();
    let response = curl.get(request_string).basic_auth(auth.token, Some(""))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();
    let data = response.bytes().await.unwrap();
    view_list(data.as_ref());
}

pub async fn list_device(id: String) {
    let auth = Configuration::get_config().unwrap();
    let request_string = format!("{}{}{}?fields=all", auth.api_uri, DEVICE, id);
    let curl = Client::new();
    let response = curl.get(request_string).basic_auth(auth.token, Some(""))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();
    let data = response.bytes().await.unwrap();
    view_device(data.as_ref());
}

pub async fn delete_device(id: String) {
    let auth = Configuration::get_config().unwrap();
    let request_string = format!("{}{}{}", auth.api_uri, DEVICE, id);
    let curl = Client::new();
    let response = curl.delete(request_string).basic_auth(auth.token, Some(""))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();
    match response.status() {
        StatusCode::OK => println!("status code: {}, successfully deleted!", response.status()),
        StatusCode::NOT_IMPLEMENTED => println!("status code: {}, cannot delete devices outside of your scope!",response.status()),
        _ => println!("wrong action!")
    }
}

pub async fn list_routes(id: String) {
    let auth = Configuration::get_config().unwrap();
    let request_string = format!("{}{}{}/routes", auth.api_uri, DEVICE, id);
    let curl = Client::new();
    let response = curl.get(request_string).basic_auth(auth.token, Some(""))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();
    let data = response.bytes().await.unwrap();
    view_routes(id, data.as_ref());
}

pub async fn set_routes(id: String, routes: String) {
    let auth = Configuration::get_config().unwrap();
    let request_string = format!("{}{}{}/routes", auth.api_uri, DEVICE, id);
    let curl = Client::new();
    let response = curl.post(request_string).basic_auth(auth.token, Some(""))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .body(routes)
        .send()
        .await
        .unwrap();
    let data = response.bytes().await.unwrap();
    view_routes(id, data.as_ref());
}

pub async fn set_authorized(id: String, authorized: String) {
    let auth = Configuration::get_config().unwrap();
    let request_string = format!("{}{}{}/authorized", auth.api_uri, DEVICE, id);
    let curl = Client::new();
    let response = curl.post(request_string).basic_auth(auth.token, Some(""))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .body(authorized)
        .send()
        .await
        .unwrap();
    match response.status() {
        StatusCode::OK => println!("status code: {}, device successfully authorized!", response.status()),
        _ => println!("something went wrong: {}", response.status()),
    }
}

pub async fn set_tags(id: String, tags: String) {
    let auth = Configuration::get_config().unwrap();
    let request_string = format!("{}{}{}/tags", auth.api_uri, DEVICE, id);
    let curl = Client::new();
    let response = curl.post(request_string).basic_auth(auth.token, Some(""))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .body(tags)
        .send()
        .await
        .unwrap();
    match response.status() {
        StatusCode::OK => println!("status code: {}, successfully assigned ACL tags on device!", response.status()),
        _ => println!("something went wrong: {}", response.status()),
    }
}

pub async fn set_key_expire(id: String, key: String, mode: bool) {
    let auth = Configuration::get_config().unwrap();
    let request_string = format!("{}{}{}/key", auth.api_uri, DEVICE, id);
    let curl = Client::new();
    let response = curl.post(request_string).basic_auth(auth.token, Some(""))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .body(key)
        .send()
        .await
        .unwrap();
    match response.status() {
        StatusCode::OK => println!("status code: {}, successfully set key expire to {}!", response.status(), mode),
        _ => println!("something went wrong: {}", response.status()),
    }
}