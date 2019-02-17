extern crate reqwest;

use std::collections::HashMap;

use reqwest::{Client, Response, Result};

use super::session::Session;

#[allow(dead_code)]
pub fn get(
    session: &Session,
    url: &'static str,
    params: Vec<(&'static str, &'static str)>,
) -> Result<Response> {
    let client = Client::new();
    client
        .get(url)
        .basic_auth(&session.api_key, Some("api_token"))
        .query(&params)
        .send()
}

#[allow(dead_code)]
pub fn post<S>(
    session: &Session,
    url: &'static str,
    params: HashMap<&str, &str>,
) -> Result<Response> {
    let client = Client::new();
    client
        .post(url)
        .basic_auth(&session.api_key, Some("api_token"))
        .json(&params)
        .send()
}
