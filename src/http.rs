extern crate reqwest;

// TODO: there's soooooooo much dupilication in here

use super::session::Session;
use reqwest::{Client, Response, Result};
use serde::Serialize;
use std::collections::HashMap;

const TOGGL_API_BASE: &str = "https://www.toggl.com/api/v8";

#[allow(dead_code)]
pub fn get(session: &Session, url: String, params: Vec<(String, String)>) -> Result<Response> {
    let full_url = get_url(url);
    let client = Client::new();
    client
        .get(&full_url)
        .basic_auth(&session.api_key, Some("api_token"))
        .query(&params)
        .send()
}

#[allow(dead_code)]
pub fn post<T: Serialize>(
    session: &Session,
    url: String,
    params: HashMap<String, T>,
) -> Result<Response> {
    let full_url = get_url(url);
    let client = Client::new();
    client
        .post(&full_url)
        .basic_auth(&session.api_key, Some("api_token"))
        .json(&params)
        .send()
}

#[allow(dead_code)]
pub fn delete(session: &Session, url: String) -> Result<Response> {
    let full_url = get_url(url);
    let client = Client::new();
    client
        .delete(&full_url)
        .basic_auth(&session.api_key, Some("api_token"))
        .send()
}

fn get_url(url: String) -> String {
    format!("{}/{}", TOGGL_API_BASE, url)
}
