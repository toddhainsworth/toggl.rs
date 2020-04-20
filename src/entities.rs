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
    pub projects: Option<Vec<Project>>,
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
        if self.projects.is_none() {
            let id = self
                .id
                .as_ref()
                .ok_or_else(|| TogglError::from("Cannot get projects for client with no ID"))?;

            let url = format!("clients/{}/projects", id);
            let mut resp =
                http::get(&session, url, Vec::new()).context("Failed to fetch client projects")?;
            self.projects = resp.json()?;
        }
        Ok(&self
            .projects
            .as_ref()
            .expect("Okay to unwrap because we ensure it's atleast an empty array"))
    }

    pub fn save() {
        // Also covers creating a new client (if self.id.is_none())
        unimplemented!();
    }

    pub fn delete(self, session: &Session) -> Result<bool, Error> {
        let id = self
            .id
            .as_ref()
            .ok_or_else(|| TogglError::from("Cannot delete client with no ID"))?;
        let url = format!("clients/{}", id);
        http::delete(session, url)
            .map(|r| r.status().is_success())
            .map_err(Error::from)
    }

    pub fn delete_by_ids(
        self,
        session: &Session,
        ids: Vec<String>,
    ) -> HashMap<String, Result<bool, Error>> {
        // TODO: not totally sure what we should return here...was thinking just a
        // Result<bool, Error> but that seems like it's too vague
        ids.into_iter()
            .map(|id| (id.clone(), Client::from_id(id).delete(session)))
            .collect()
    }

    // Util function to create a default client from the given id.
    // Note; no request to Toggl
    pub fn from_id(id: String) -> Self {
        let mut client = Client::default();
        client.id = Some(id);
        client
    }
}

// Projects ------------------------------------------------------------------------;
#[derive(Serialize, Deserialize, Debug)]
struct ProjectData {
    data: Project,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Project {
    pub id: Option<String>,
    pub wid: Option<String>,
    pub cid: Option<String>,
    pub name: String,
    pub billable: bool,
    pub is_private: bool,
    pub active: bool,
    pub at: Option<String>,
    pub template: bool,
    pub color: String,
    pub users: Option<Vec<User>>,
    pub tasks: Option<Vec<Task>>,
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

    pub fn delete(self, session: &Session) -> Result<bool, Error> {
        let id = self
            .id
            .as_ref()
            .ok_or_else(|| TogglError::from("Cannot delete project with no ID"))?;
        let url = format!("projects/{}", id);
        http::delete(session, url)
            .map(|r| r.status().is_success())
            .map_err(Error::from)
    }

    pub fn delete_by_ids(self, session: &Session, ids: Vec<String>) -> Result<(), Error> {
        // TODO: not totally sure what we should return here...was thinking just a
        // Result<bool, Error> but that seems like it's too vague
        let url = format!("projects/{}", ids.join(","));
        http::delete(session, url).map(|_| ()).map_err(Error::from)
    }

    pub fn users(&mut self, session: &Session) -> Result<&Vec<User>, Error> {
        if self.users.is_none() {
            let id = self
                .id
                .as_ref()
                .ok_or_else(|| TogglError::from("Cannot get users for project with no ID"))?;

            let url = format!("projects/{}/users", id);
            let mut resp =
                http::get(&session, url, Vec::new()).context("Failed to fetch project users")?;
            let users: Vec<User> = resp.json()?;
            self.users = Some(users);
        }
        Ok(&self
            .users
            .as_ref()
            .expect("Okay to unwrap because we ensure it's atleast an empty array"))
    }

    pub fn tasks(&mut self, session: &Session) -> Result<&Vec<Task>, Error> {
        if self.tasks.is_none() {
            let id = self
                .id
                .as_ref()
                .ok_or_else(|| TogglError::from("Cannot get tasks for project with no ID"))?;

            let url = format!("projects/{}/tasks", id);
            let mut resp =
                http::get(&session, url, Vec::new()).context("Failed to fetch project tasks")?;
            let tasks: Vec<Task> = resp.json()?;
            self.tasks = Some(tasks);
        }
        Ok(&self
            .tasks
            .as_ref()
            .expect("Okay to unwrap because we ensure it's atleast an empty array"))
    }

    // Util function to create a default client from the given id.
    // Note; no request to Toggl
    pub fn from_id(id: String) -> Self {
        let mut project = Project::default();
        project.id = Some(id);
        project
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
    pub clients: Option<Vec<Client>>,
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
            let id = self
                .id
                .as_ref()
                .ok_or_else(|| TogglError::from("Cannot get projects for client with no ID"))?;

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

    pub fn tasks(&mut self, session: &Session) -> Result<&Vec<Task>, Error> {
        if self.tasks.is_none() {
            let id = self
                .id
                .as_ref()
                .ok_or_else(|| TogglError::from("Cannot get tasks for client with no ID"))?;

            let url = format!("workspaces/{}/tasks", id);
            let mut resp =
                http::get(&session, url, Vec::new()).context("Failed to fetch workspace tasks")?;
            let tasks: Vec<Task> = resp.json()?;
            self.tasks = Some(tasks);
        }
        Ok(&self
            .tasks
            .as_ref()
            .expect("Okay to unwrap because we ensure it's atleast an empty array"))
    }

    pub fn tags(&mut self, session: &Session) -> Result<&Vec<Tag>, Error> {
        if self.tags.is_none() {
            let id = self
                .id
                .as_ref()
                .ok_or_else(|| TogglError::from("Cannot get tags for client with no ID"))?;

            let url = format!("workspaces/{}/tags", id);
            let mut resp =
                http::get(&session, url, Vec::new()).context("Failed to fetch workspace tags")?;
            let tags: Vec<Tag> = resp.json()?;
            self.tags = Some(tags);
        }
        Ok(&self
            .tags
            .as_ref()
            .expect("Okay to unwrap because we ensure it's atleast an empty array"))
    }

    pub fn clients(&mut self, session: &Session) -> Result<&Vec<Client>, Error> {
        if self.clients.is_none() {
            let id = self
                .id
                .as_ref()
                .ok_or_else(|| TogglError::from("Cannot get clients for client with no ID"))?;

            let url = format!("workspaces/{}/clients", id);
            let mut resp = http::get(&session, url, Vec::new())
                .context("Failed to fetch workspace clients")?;
            let clients: Vec<Client> = resp.json()?;
            self.clients = Some(clients);
        }
        Ok(&self
            .clients
            .as_ref()
            .expect("Okay to unwrap because we ensure it's atleast an empty array"))
    }

    pub fn groups(&mut self, session: &Session) -> Result<&Vec<Group>, Error> {
        if self.groups.is_none() {
            let id = self
                .id
                .as_ref()
                .ok_or_else(|| TogglError::from("Cannot get groups for client with no ID"))?;

            let url = format!("workspaces/{}/groups", id);
            let mut resp =
                http::get(&session, url, Vec::new()).context("Failed to fetch workspace groups")?;
            let groups: Vec<Group> = resp.json()?;
            self.groups = Some(groups);
        }
        Ok(&self
            .groups
            .as_ref()
            .expect("Okay to unwrap because we ensure it's atleast an empty array"))
    }

    pub fn project_users(&mut self, session: &Session) -> Result<&Vec<ProjectUser>, Error> {
        if self.project_users.is_none() {
            let id = self.id.as_ref().ok_or_else(|| {
                TogglError::from("Cannot get project users for client with no ID")
            })?;

            let url = format!("workspaces/{}/project_users", id);
            let mut resp = http::get(&session, url, Vec::new())
                .context("Failed to fetch workspace project_users")?;
            let project_users: Vec<ProjectUser> = resp.json()?;
            self.project_users = Some(project_users);
        }
        Ok(&self
            .project_users
            .as_ref()
            .expect("Okay to unwrap because we ensure it's atleast an empty array"))
    }

    pub fn workspace_users(&mut self, session: &Session) -> Result<&Vec<WorkspaceUser>, Error> {
        if self.workspace_users.is_none() {
            let id = self.id.as_ref().ok_or_else(|| {
                TogglError::from("Cannot get workspace users for client with no ID")
            })?;

            let url = format!("workspaces/{}/workspace_users", id);
            let mut resp = http::get(&session, url, Vec::new())
                .context("Failed to fetch workspace workspace_users")?;
            let workspace_users: Vec<WorkspaceUser> = resp.json()?;
            self.workspace_users = Some(workspace_users);
        }
        Ok(&self
            .workspace_users
            .as_ref()
            .expect("Okay to unwrap because we ensure it's atleast an empty array"))
    }

    pub fn invite_user(
        &self,
        session: &Session,
        emails: Vec<String>,
    ) -> Result<Vec<WorkspaceUser>, Error> {
        let id = self
            .id
            .as_ref()
            .ok_or_else(|| TogglError::from("Cannot invite users to a workspace that has no ID"))?;
        let url = format!("workspaces/{}/invite", id);

        let mut data: HashMap<String, Vec<String>> = HashMap::new();
        data.insert("emails".to_string(), emails);

        let mut resp =
            http::post(&session, url, data).context("Failed to invite users to workspace")?;
        let workspace_users: Vec<WorkspaceUser> = resp.json()?;
        Ok(workspace_users)
    }
}

// Time Entries ------------------------------------------------------------------------;

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeEntryData {
    pub data: TimeEntry,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TimeEntry {
    pub id: Option<String>,
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
    pub fn get(session: &Session, id: &str) -> Result<Self, Error> {
        let url = format!("time_entries/{}", id);
        let mut resp = http::get(&session, url, vec![])
            .context(format!("Failed to fetch time entry with ID {}", id))?;
        resp.json().map_err(Error::from)
    }

    // todo: convert these to chrono dates to enforce format
    pub fn get_in_range(
        session: &Session,
        start_date: &str,
        end_date: &str,
    ) -> Result<Self, Error> {
        let url = "time_entries".to_string();
        let mut resp = http::get(
            &session,
            url,
            vec![
                ("start_date".to_string(), start_date.to_string()),
                ("end_date".to_string(), end_date.to_string()),
            ],
        )
        .context(format!(
            "Failed to fetch time entries between {} and {}",
            start_date, end_date
        ))?;
        resp.json().map_err(Error::from)
    }

    pub fn get_running(session: &Session) -> Result<Self, Error> {
        let url = "time_entries/current".to_string();
        let mut resp =
            http::get(&session, url, vec![]).context("Failed to fetch current time entry")?;
        resp.json().map_err(Error::from)
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

    pub fn delete(self, session: &Session) -> Result<bool, Error> {
        let id = self
            .id
            .as_ref()
            .ok_or_else(|| TogglError::from("Cannot delete time entry with no ID"))?;
        let url = format!("time_entries/{}", id);
        http::delete(session, url)
            .map(|r| r.status().is_success())
            .map_err(Error::from)
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
