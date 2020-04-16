use super::session::Session;

use std::collections::HashMap;
use std::fmt::Debug;

use failure::{Error, ResultExt};

// Users ------------------------------------------------------------------------------------;

// FIXME: I'm not overly fond of this...
#[derive(Serialize, Deserialize, Debug)]
struct UserData {
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
    pub at: String,                             // TODO: make me a chrono::DateTime
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
    pub fn me(session: &Session) -> Result<Self, Error> {
        // TODO: better messaging?
        let mut resp = super::http::get(
            session,
            "me".to_string(),
            vec![("with_related_data".to_string(), "false".to_string())],
        )
        .context("Failed to get from the \"me\" API")?;
        let body: UserData = resp.json().context("Failed to deserialize user data")?;
        Ok(body.data)
    }
}

// Clients ------------------------------------------------------------------------------------;

// FIXME: I'm not overly fond of this...
#[derive(Serialize, Deserialize, Debug)]
struct ClientData {
    pub data: Client,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Client {
    pub id: usize,
    pub name: String,
    pub wid: usize,
    pub notes: Option<String>,
    pub at: Option<String>,
}

impl Client {
    // TOOD: Write a test that doesn't hit the API for me :)
    pub fn all(session: &Session) -> Result<Vec<Self>, Error> {
        let mut resp = super::http::get(&session, "clients".to_string(), Vec::new())
            .context("Failed to fetch clients for this user")?;
        resp.json().map_err(Error::from)
    }

    // TOOD: Write a test that doesn't hit the API for me :)
    pub fn get(session: &Session, id: &str) -> Result<Self, Error> {
        let url = format!("clients/{}", id);
        let mut resp = super::http::get(&session, url, vec![])
            .context(format!("Failed to fetch client with ID {}", id))?;
        resp.json().map_err(Error::from)
    }

    pub fn projects(&self, session: &Session) -> Result<Project, Error> {
        let url = format!("clients/{}/projects", self.id);
        let mut resp = super::http::get(&session, url, Vec::new())
            .context("Failed to fetch client projects")?;
        resp.json().map_err(Error::from)
    }
}

impl Default for Client {
    fn default() -> Self {
        Client {
            id: 0,
            name: String::default(),
            wid: 0,
            at: None,
            notes: None,
        }
    }
}

// Projects ------------------------------------------------------------------------;
#[derive(Serialize, Deserialize, Debug)]
struct ProjectData {
    data: Project,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub id: usize,
    pub wid: usize,
    pub cid: usize,
    pub name: String,
    pub billable: bool,
    pub is_private: bool,
    pub active: bool,
    pub at: Option<String>,
    pub template: bool,
    pub color: String,
}

impl Project {
    pub fn get(session: &Session, id: &str) -> Result<Self, Error> {
        let url = format!("projects/{}", id);
        let mut resp =
            super::http::get(&session, url, Vec::new()).context("Failed to fetch project")?;
        let body: ProjectData = resp.json().context("Failed to parse Project from JSON")?;
        Ok(body.data)
    }
}

impl Default for Project {
    fn default() -> Self {
        Project {
            id: 0,
            wid: 0,
            cid: 0,
            name: String::default(),
            billable: false,
            is_private: false,
            active: true,
            at: None,
            template: false,
            color: String::default(),
        }
    }
}
