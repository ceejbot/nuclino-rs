//! This is a Rust module-level documentation string. It documents the module.
//! This is a flammenwerfer. It werfs flammen.

#![deny(future_incompatible, clippy::unwrap_used)]
#![warn(rust_2018_idioms, trivial_casts)]

pub mod errors;
mod request_types;
mod response_types;
mod types;

use errors::*;
pub use request_types::*;
use response_types::*;
pub use types::*;

use serde::{Deserialize, Serialize};
use urlencoding::encode;
pub use uuid::Uuid;

/// The base url for the entire API.
pub static BASE_URL: &str = "https://api.nuclino.com";

/// The env var we check for the api key.
pub static APIKEY_ENV_VAR: &str = "NUCLINO_API_KEY";

/// A client for the Nuclino api. This struct maintains whatever state we need
/// for making requests as a specific user. The functions provided are conveniences
/// for accessing endpoints in the [official Nuclino API](https://help.nuclino.com/d3a29686-api).
pub struct Client {
    apikey: String,
    baseurl: String,
    client: ureq::Agent,
}

impl Client {
    /// Create a client, passing in the api key you want to use, and a base url if you
    /// want to override the default.
    pub fn create(apikey: &str, base_url: Option<&str>) -> Self {
        let client = ureq::AgentBuilder::new()
            .https_only(true)
            .user_agent("ceejbot/nuclino-rs")
            .build();
        let baseurl = if let Some(base) = base_url {
            base.to_owned()
        } else {
            BASE_URL.to_owned()
        };

        Client {
            apikey: apikey.to_owned(),
            baseurl,
            client,
        }
    }

    /// Create a Nuclino client with an API key read from the env var `NUCLINO_API_KEY`
    /// using the default base url.
    pub fn create_from_env() -> NuclinoResult<Self> {
        let Ok(key) = std::env::var(APIKEY_ENV_VAR) else {
            return Err(NuclinoError::ApiKeyNotFound);
        };
        Ok(Client::create(key.as_str(), None))
    }

    /// Fetch a single user by id.
    pub fn user(&self, id: &Uuid) -> NuclinoResult<User> {
        self.get(format!("{}/v0/users/{id}", self.baseurl))
    }

    /// Fetch a list of teams, optionally paginated.
    pub fn team_list(&self, limit: Option<u8>, after: Option<&str>) -> NuclinoResult<Vec<Team>> {
        // ureq doesn't handle query params for us so let's hack this up fast.
        let mut query: Vec<String> = vec![];
        if let Some(max) = limit {
            query.push(format!("limit={max}"));
        }
        if let Some(prev) = after {
            if !query.is_empty() {
                query.push("&".to_string());
            }
            query.push(format!("after={prev}"));
        }

        let url = if !query.is_empty() {
            format!("{}/v0/teams?{}", self.baseurl, query.join(""))
        } else {
            format!("{}/v0/teams", self.baseurl)
        };
        let result = self.get::<List<Team>>(url)?;
        Ok(result.as_vec())
    }

    /// Fetch a single team by id.
    pub fn team(&self, id: &str) -> NuclinoResult<Team> {
        self.get(format!("{}/v0/teams/{id}", self.baseurl))
    }

    /// Fetch a list of workspaces, optionally paginated.
    pub fn workspace_list(
        &self,
        limit: Option<usize>,
        after: Option<&str>,
    ) -> NuclinoResult<Vec<Workspace>> {
        // GET /v0/workspaces
        let mut query: Vec<String> = vec![];
        if let Some(max) = limit {
            query.push(format!("limit={max}"));
        }
        if let Some(prev) = after {
            if !query.is_empty() {
                query.push("&".to_string());
            }
            query.push(format!("after={prev}"));
        }

        let url = if !query.is_empty() {
            format!("{}/v0/workspaces?{}", self.baseurl, query.join(""))
        } else {
            format!("{}/v0/workspaces", self.baseurl)
        };
        let result = self.get::<List<Workspace>>(url)?;
        Ok(result.as_vec())
    }

    /// Fetch a single workspace by id.
    pub fn workspace(&self, id: &Uuid) -> NuclinoResult<Workspace> {
        self.get::<Workspace>(format!("{}/v0/workspaces/{id}", self.baseurl))
    }

    /// Create a Nuclino page, which might be either an item or a collection.
    pub fn page_create(&self, page: NewPage) -> NuclinoResult<Page> {
        self.post::<Page>(format!("{}/v0/items", self.baseurl), page)
    }

    /// Fetch a Nuclino page by id.
    pub fn page(&self, id: &Uuid) -> NuclinoResult<Page> {
        self.get::<Page>(format!("{}/v0/items/{id}", self.baseurl))
    }

    /// Update item or collection
    pub fn page_update(&self, id: &Uuid, updated: &ModifyItem) -> NuclinoResult<Page> {
        self.put::<Page>(format!("{}/v0/items/{id}", self.baseurl), updated)
    }

    /// Delete an item or collection by id.
    pub fn page_delete(&self, id: &Uuid) -> NuclinoResult<IdOnly> {
        self.delete::<IdOnly>(format!("{}/v0/items/{id}", self.baseurl))
    }

    /// Get all items and collections belonging to a single team, _without_ page content.
    /// `limit` defaults to 100 in the Nuclino api if not provided. To fetch the next set
    /// of pages in a paginated list, provide the id of the last item in the current page
    /// in the `after` param.
    pub fn all_pages_for_team(
        &self,
        team: &Uuid,
        limit: Option<u8>,
        after: Option<&Uuid>,
    ) -> NuclinoResult<List<Page>> {
        // ureq doesn't handle query params for us so let's hack this up fast.
        let mut query: Vec<String> = vec!["?".to_string()];
        query.push(format!("teamId={team}"));
        if let Some(lim) = limit {
            query.push(format!("&limit={lim}"));
        }
        if let Some(id) = after {
            query.push(format!("&limit={id}"))
        }
        let url = format!("{}/v0/items{}", self.baseurl, query.join(""));
        self.get::<List<Page>>(url)
    }

    /// Get all items and collections belonging to a single workspace, _without_ page content.
    /// `limit` defaults to 100 in the Nuclino api if not provided. To fetch the next set
    /// of pages in a paginated list, provide the id of the last item in the current page
    /// in the `after` param.
    pub fn all_pages_for_workspace(
        &self,
        workspace: &Uuid,
        limit: Option<u8>,
        after: Option<&Uuid>,
    ) -> NuclinoResult<List<Page>> {
        let mut query: Vec<String> = vec!["?".to_string()];
        query.push(format!("workspaceId={workspace}"));
        if let Some(lim) = limit {
            query.push(format!("&limit={lim}"));
        }
        if let Some(id) = after {
            query.push(format!("&limit={id}"))
        }
        let url = format!("{}/v0/items{}", self.baseurl, query.join(""));
        self.get::<List<Page>>(url)
    }

    /// Search a team's pages for the given text. Returns a list of pages without content.
    /// Pass `limit` to restrict the number of results returned; the default number returned
    /// by the server is 100.
    pub fn search_team(
        &self,
        team: &Uuid,
        search: &str,
        limit: Option<u8>,
    ) -> NuclinoResult<Vec<Page>> {
        let mut query: Vec<String> = vec![];
        query.push(format!("?teamId={team}"));
        query.push(format!("&search={}", encode(search)));
        if let Some(max) = limit {
            query.push(format!("&limit={max}"));
        }
        let url = format!("{}/v0/items{}", self.baseurl, query.join(""));
        let list = self.get::<List<Page>>(url)?;
        Ok(list.as_vec())
    }

    /// Search a workspace's pages for the given text. Returns a list of pages without content.
    /// Pass `limit` to restrict the number of results returned; the default number returned
    /// by the server is 100.
    pub fn search_workspace(
        &self,
        workspace: &Uuid,
        search: &str,
        limit: Option<u8>,
    ) -> NuclinoResult<Vec<Page>> {
        let mut query: Vec<String> = vec![];
        query.push(format!("?workspaceId={workspace}"));
        query.push(format!("&search={}", encode(search)));
        if let Some(max) = limit {
            query.push(format!("&limit={max}"));
        }
        let url = format!("{}/v0/items{}", self.baseurl, query.join(""));
        let list = self.get::<List<Page>>(url)?;
        Ok(list.as_vec())
    }

    // TODO extract commonalities

    /// Internal details of the `GET` implementation.
    fn get<T>(&self, path: String) -> NuclinoResult<T>
    where
        T: for<'de> Deserialize<'de> + Clone,
    {
        let response = self
            .client
            .get(path.as_str())
            .set("Authorization", &self.apikey)
            .call()?;
        let status = response.status();
        let body: Response<T> = response.into_json::<Response<T>>()?;
        if body.is_success() {
            if let Some(data) = body.data() {
                Ok(data.clone())
            } else {
                Err(NuclinoError::NoDataReturned)
            }
        } else {
            Err(make_error(status, body.message()))
        }
    }

    fn put<T>(&self, path: String, payload: impl Serialize) -> NuclinoResult<T>
    where
        T: for<'de> Deserialize<'de> + Clone,
    {
        let response = self
            .client
            .put(path.as_str())
            .set("Authorization", &self.apikey)
            .send_json(payload)?;
        let status = response.status();
        let body: Response<T> = response.into_json()?;
        if body.is_success() {
            if let Some(data) = body.data() {
                Ok(data.clone())
            } else {
                Err(NuclinoError::NoDataReturned)
            }
        } else {
            Err(make_error(status, body.message()))
        }
    }

    fn post<T>(&self, path: String, payload: impl Serialize) -> NuclinoResult<T>
    where
        T: for<'de> Deserialize<'de> + Clone,
    {
        let response = self
            .client
            .post(path.as_str())
            .set("Authorization", &self.apikey)
            .send_json(payload)?;
        let status = response.status();
        let body: Response<T> = response.into_json()?;
        if body.is_success() {
            if let Some(data) = body.data() {
                Ok(data.clone())
            } else {
                Err(NuclinoError::NoDataReturned)
            }
        } else {
            Err(make_error(status, body.message()))
        }
    }

    fn delete<T>(&self, path: String) -> NuclinoResult<T>
    where
        T: for<'de> Deserialize<'de> + Clone,
    {
        let response = self
            .client
            .delete(path.as_str())
            .set("Authorization", &self.apikey)
            .call()?;
        let status = response.status();
        let body: Response<T> = response.into_json()?;
        if body.is_success() {
            if let Some(data) = body.data() {
                Ok(data.clone())
            } else {
                Err(NuclinoError::NoDataReturned)
            }
        } else {
            Err(make_error(status, body.message()))
        }
    }
}
