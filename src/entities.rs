use super::session::Session;

use std::collections::HashMap;

// FIXME: I'm not overly fond of this...
#[derive(Serialize, Deserialize)]
pub struct UserData {
    pub since: usize,
    pub data: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub api_token: String,
    pub default_wid: usize,
    pub email: String,
    pub fullname: String,
    pub jquery_timeofday_format: String,
    pub jquery_date_format: String,
    pub timeofday_format: String,
    pub date_format: String,
    pub store_start_and_stop_time: bool,
    pub beginning_of_week: usize,
    pub language: String,
    pub image_url: String,
    pub sidebar_piechart: bool,
    pub at: String, // API docs don't detail what this type should be
    pub new_blog_post: HashMap<String, String>, // TODO
    pub send_product_emails: bool,
    pub send_timer_notifications: bool,
    pub openid_enabled: bool,
    pub timezone: String,
}

impl Default for User {
    fn default() -> Self {
        User {
            api_token: String::default(),
            default_wid: 0,
            email: String::default(),
            fullname: String::default(),
            jquery_timeofday_format: String::default(),
            jquery_date_format: String::default(),
            timeofday_format: String::default(),
            date_format: String::default(),
            store_start_and_stop_time: false,
            beginning_of_week: 0,
            language: String::default(),
            image_url: String::default(),
            sidebar_piechart: false,
            at: String::default(),
            new_blog_post: HashMap::new(), // TODO
            send_product_emails: false,
            send_timer_notifications: false,
            openid_enabled: false,
            timezone: String::default(),
        }
    }
}

impl User {
    pub fn me(session: &Session) -> Result<Self, reqwest::Error> {
        let mut resp = super::http::get(session, "me", Vec::new())?;
        let body: UserData = resp.json()?;
        return Ok(body.data);
    }
}
