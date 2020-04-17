use super::error::TogglError;
use super::http;
use super::session::Session;
use failure::{Error, ResultExt};
use std::collections::HashMap;
use std::fmt::Debug;

// Users ------------------------------------------------------------------------------------;

// FIXME: I'm not overly fond of this...
#[derive(Serialize, Deserialize, Debug)]
struct UserData {
    pub since: usize,
    pub data: User,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct User {
    pub id: Option<String>,
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
    pub at: Option<String>, // TODO: make me a chrono::DateTime
    pub new_blog_post: HashMap<String, String>, // TODO
    pub send_product_emails: bool,
    pub send_timer_notifications: bool,
    pub openid_enabled: bool,
    pub timezone: String,
    pub invitation: HashMap<String, String>, // TODO
}

impl User {
    pub fn me(session: &Session) -> Result<Self, Error> {
        let mut resp = http::get(
            session,
            "me".to_string(),
            vec![("with_related_data".to_string(), "false".to_string())],
        )
        .context("Failed to get from the \"me\" API")?;
        let body: UserData = resp.json().context("Failed to deserialize user data")?;
        Ok(body.data)
    }

    pub fn save() {
        // Also covers signing up new user (if self.id.is_none())
        unimplemented!();
    }

    pub fn reset_token() {
        unimplemented!();
    }
}

// Clients ------------------------------------------------------------------------------------;

// FIXME: I'm not overly fond of this...
#[derive(Serialize, Deserialize, Debug)]
struct ClientData {
    pub data: Client,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Client {
    pub id: Option<String>,
    pub name: String,
    pub wid: Option<usize>,
    pub notes: Option<String>,
    pub at: Option<String>,
    pub projects: Vec<Project>,
}

impl Client {
    // TOOD: Write a test that doesn't hit the API for me :)
    pub fn all(session: &Session) -> Result<Vec<Self>, Error> {
        let mut resp = http::get(&session, "clients".to_string(), Vec::new())
            .context("Failed to fetch clients for this user")?;
        resp.json().map_err(Error::from)
    }

    // TOOD: Write a test that doesn't hit the API for me :)
    pub fn get(session: &Session, id: &str) -> Result<Self, Error> {
        let url = format!("clients/{}", id);
        let mut resp = http::get(&session, url, vec![])
            .context(format!("Failed to fetch client with ID {}", id))?;
        resp.json().map_err(Error::from)
    }

    pub fn projects(&mut self, session: &Session) -> Result<&Vec<Project>, Error> {
        let id = self.id.as_ref().ok_or(TogglError::from(
            "Cannot get projects for client with no ID",
        ))?;

        if self.projects.is_empty() {
            let url = format!("clients/{}/projects", id);
            let mut resp =
                http::get(&session, url, Vec::new()).context("Failed to fetch client projects")?;
            self.projects = resp.json()?;
        }
        Ok(&self.projects)
    }

    pub fn save() {
        // Also covers creating a new client (if self.id.is_none())
        unimplemented!();
    }

    pub fn delete() {
        unimplemented!();
    }

    pub fn delete_by_ids() {
        unimplemented!();
    }
}

// Projects ------------------------------------------------------------------------;
#[derive(Serialize, Deserialize, Debug)]
struct ProjectData {
    data: Project,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Project {
    pub id: Option<usize>,
    pub wid: Option<usize>,
    pub cid: Option<usize>,
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
        let mut resp = http::get(&session, url, Vec::new()).context("Failed to fetch project")?;
        let body: ProjectData = resp.json().context("Failed to parse Project from JSON")?;
        Ok(body.data)
    }

    pub fn save() {
        // Also covers creating new project (if self.id.is_none())
        unimplemented!();
    }

    pub fn delete() {
        unimplemented!();
    }

    pub fn delete_by_ids() {
        unimplemented!();
    }

    pub fn users() {
        unimplemented!();
    }

    pub fn tasks() {
        unimplemented!();
    }
}

// Workspaces ------------------------------------------------------------------------;

#[derive(Serialize, Deserialize, Debug)]
struct WorkspaceData {
    data: Workspace,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Workspace {
    pub id: Option<String>,
    pub name: String,
    pub premium: bool,
    pub admin: bool,
    pub default_hourly_rate: usize,
    pub default_currency: String,
    pub only_admins_may_create_projects: bool,
    pub only_admins_see_billable_rates: bool,
    pub rounding: usize,
    pub rounding_minutes: usize,
    pub at: Option<String>,
    pub logo_url: Option<String>,
    pub projects: Option<Vec<Project>>,
    pub tasks: Option<Vec<Task>>,
    pub tags: Option<Vec<Tag>>,
    pub groups: Option<Vec<Group>>,
    pub project_users: Option<Vec<ProjectUser>>,
    pub workspace_users: Option<Vec<WorkspaceUser>>,
}

impl Workspace {
    pub fn all(session: &Session) -> Result<Vec<Self>, Error> {
        let mut resp = http::get(&session, "workspaces".to_string(), Vec::new())
            .context("Failed to fetch all workspaces")?;
        resp.json().map_err(Error::from)
    }

    pub fn get(session: &Session, id: &str) -> Result<Self, Error> {
        let url = format!("workspaces/{}", id);
        let mut resp = http::get(&session, url, Vec::new()).context("Failed to fetch workspace")?;
        let body: WorkspaceData = resp.json().context("Failed to parse workspace from JSON")?;
        Ok(body.data)
    }

    pub fn projects(&mut self, session: &Session) -> Result<&Vec<Project>, Error> {
        if self.projects.is_none() {
            let id = self.id.as_ref().ok_or(TogglError::from(
                "Cannot get projects for client with no ID",
            ))?;

            let url = format!("workspaces/{}/projects", id);
            let mut resp = http::get(&session, url, Vec::new())
                .context("Failed to fetch workspace projects")?;
            let projects: Vec<Project> = resp.json()?;
            self.projects = Some(projects);
        }
        Ok(&self
            .projects
            .as_ref()
            .expect("Okay to unwrap because we ensure it's atleast an empty array"))
    }

    pub fn tasks() {
        unimplemented!();
    }

    pub fn tags() {
        unimplemented!();
    }

    pub fn clients() {
        unimplemented!();
    }

    pub fn groups() {
        unimplemented!();
    }

    pub fn project_users() {
        unimplemented!();
    }

    pub fn workspace_users() {
        unimplemented!();
    }

    pub fn invite_user() {
        unimplemented!();
    }
}

// Time Entries ------------------------------------------------------------------------;

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeEntryData {
    pub data: TimeEntry,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TimeEntry {
    pub description: String,
    pub wid: Option<String>,
    pub pid: Option<String>,
    pub tid: Option<String>,
    pub billable: bool,
    pub start: String,
    pub stop: Option<String>,
    pub duration: usize,
    pub created_with: String,
    pub tags: Vec<String>,
    pub duronly: bool,
    pub at: Option<String>,
}

impl TimeEntry {
    pub fn get() {
        unimplemented!();
    }

    // get between two dates
    pub fn get_in_range() {
        unimplemented!();
    }

    pub fn get_running() {
        unimplemented!();
    }

    pub fn save() {
        // updates existing or creates a new one (if self.id.is_none())
        unimplemented!();
    }

    // does not take `self` instead takes a Vec<TimeEntry> and pushes to the bulk API
    pub fn bulk_save() {
        unimplemented!();
    }

    pub fn start() {
        unimplemented!();
    }

    pub fn stop() {
        unimplemented!();
    }

    pub fn delete() {
        unimplemented!();
    }
}

// Groups ---------------------------------------------------------------------------------------;

#[derive(Serialize, Deserialize, Debug)]
pub struct GroupData {
    pub data: Group,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Group {
    pub id: Option<String>,
    pub name: String,
    pub wid: String,
    pub at: Option<String>, // TODO: chronos::DateTime
}

impl Group {
    // Doesn't seem to offer a get API
    pub fn save() {
        // will update existing or create new (if self.id.is_none())
        unimplemented!();
    }

    pub fn delete() {
        unimplemented!();
    }

    pub fn delete_by_ids() {
        unimplemented!();
    }
}

// Project Users ---------------------------------------------------------------------------------;
#[derive(Serialize, Deserialize, Debug)]
struct ProjectUsersData {
    data: ProjectUser,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ProjectUser {
    pub id: Option<String>,
    pub pid: String,
    pub uid: String,
    pub wid: Option<String>,
    pub manager: bool,
    pub rate: f64,
    pub at: Option<String>,
}

impl ProjectUser {
    pub fn save() {
        // will update existing or create new project user (if self.id.is_none())
        unimplemented!();
    }

    pub fn mass_save() {
        // will update existing only
        unimplemented!();
    }

    pub fn delete() {
        unimplemented!();
    }

    pub fn delete_by_ids() {
        unimplemented!();
    }

    pub fn create_in_project() {
        unimplemented!();
    }
}

// Tags ------------------------------------------------------------------------------------------;
#[derive(Serialize, Deserialize, Debug)]
struct TagData {
    pub data: Tag,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Tag {
    id: Option<String>,
    wid: String,
    name: String,
}

impl Tag {
    pub fn save() {
        // will update existing tags or create new ones (if self.id.is_none())
        unimplemented!();
    }

    pub fn delete() {
        unimplemented!();
    }

    pub fn delete_by_ids() {
        unimplemented!();
    }
}

// Tasks ------------------------------------------------------------------------------------------;
#[derive(Serialize, Deserialize, Debug)]
struct TaskData {
    pub data: Task,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Task {
    pub id: Option<String>,
    pub name: String,
    pub wid: Option<String>,
    pub pid: String,
    pub active: bool,
    pub estimated_seconds: usize,
}

impl Task {
    pub fn get() {
        unimplemented!();
    }

    pub fn save() {
        // updates existing task or creates a new one (if self.id.is_none())
        unimplemented!();
    }

    pub fn mass_save() {
        // will update existing only
        unimplemented!();
    }

    pub fn delete() {
        unimplemented!();
    }

    pub fn delete_by_ids() {
        unimplemented!();
    }
}

// Workspace Users --------------------------------------------------------------------------------;
#[derive(Serialize, Deserialize, Debug)]
struct WorkspaceUserData {
    pub data: WorkspaceUser,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct WorkspaceUser {}

impl WorkspaceUser {
    pub fn save() {
        // will update existing workspace user or create new (if self.id.is_none())
        unimplemented!();
    }

    pub fn delete() {
        unimplemented!();
    }

    pub fn delete_by_id() {
        unimplemented!();
    }
}
