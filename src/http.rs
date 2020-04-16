extern crate reqwest;

use std::collections::HashMap;

use reqwest::{Client, Response, Result};

use super::session::Session;

const TOGGL_API_BASE: &str = "https://www.toggl.com/api/v8";

#[allow(dead_code)]
pub fn get(
    session: &Session,
    url: String,
    params: Vec<(&'static str, &'static str)>,
) -> Result<Response> {
    let full_url = get_url(url);
    let client = Client::new();
    client
        .get(&full_url)
        .basic_auth(&session.api_key, Some("api_token"))
        .query(&params)
        .send()
}

#[allow(dead_code)]
pub fn post(session: &Session, url: String, params: HashMap<&str, &str>) -> Result<Response> {
    let full_url = get_url(url);
    let client = Client::new();
    client
        .post(&full_url)
        .basic_auth(&session.api_key, Some("api_token"))
        .json(&params)
        .send()
}

fn get_url(url: String) -> String {
    format!("{}/{}", TOGGL_API_BASE, url)
}
