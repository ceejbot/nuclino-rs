//! Nuclino data types exposed by its API, and traits on those types.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;

/// An id-only response structure, returned by `DELETE` endpoints.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IdOnly {
    id: Uuid,
}

impl IdOnly {
    /// Get the id of this data stub.
    pub fn id(&self) -> &Uuid {
        &self.id
    }
}

/// A Nuclino user.
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    avatar_url: Option<String>,
}

impl User {
    /// The ID of this user.
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// This user's first name.
    pub fn first_name(&self) -> &str {
        self.first_name.as_str()
    }

    /// This user's last name.
    pub fn last_name(&self) -> &str {
        self.last_name.as_str()
    }

    /// This user's email address.
    pub fn email(&self) -> &str {
        self.email.as_str()
    }

    /// A url for this user's avatar.
    pub fn avatar_url(&self) -> Option<&String> {
        self.avatar_url.as_ref()
    }
}

/// A Nuclino team.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    id: Uuid,
    url: String,
    name: String,
    created_at: String,
    created_user_id: Uuid,
}

impl Team {
    /// The ID of this team.
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Creation timestamp as an ISO-8601 string
    pub fn created(&self) -> &str {
        self.created_at.as_str()
    }

    /// The ID of the user who created this team.
    pub fn created_by(&self) -> &Uuid {
        &self.created_user_id
    }

    /// This team's url.
    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    /// This team's name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

/// A workspace.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Workspace {
    id: Uuid,
    team_id: Uuid,
    name: String,
    created_at: String, // date
    created_user_id: Uuid,
    fields: Vec<Field>,
    child_ids: Vec<Uuid>,
}

impl Workspace {
    /// All directly-accessible items in in the Nuclino API have UUID ids.
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Creation timestamp as an ISO-8601 string
    pub fn created(&self) -> &str {
        self.created_at.as_str()
    }

    /// The ID of the user who created this workspace.
    pub fn created_by(&self) -> &Uuid {
        &self.created_user_id
    }

    /// The ID of the owning team.
    pub fn team_id(&self) -> &Uuid {
        &self.team_id
    }

    /// This workspace's name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Metadata for the fields provided as metadata for pages in this workspace.
    pub fn fields(&self) -> &[Field] {
        self.fields.as_slice()
    }

    /// Ids of the child pages of this workspace.
    pub fn children(&self) -> &[Uuid] {
        self.child_ids.as_slice()
    }
}

/// `Fields` at the workspace level are metadata describing what
/// metadata a single page can have.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    id: Uuid,
    name: String,
    #[serde(default)]
    config: Config,
    #[serde(rename = "type")]
    field_type: FieldType,
}

impl Field {
    /// The field's ID.
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// The field's name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// What kind of field this is describing.
    pub fn field_type(&self) -> &FieldType {
        &self.field_type
    }

    /// What this field's configuration is. The enum represents each category of config
    /// that the API exposes. Select & multiselect fields use the same config type, as
    /// do the timestamp field varieties.
    pub fn configuration(&self) -> &Config {
        &self.config
    }
}

/// What kind of config this field meta object has.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase", untagged)]
pub enum Config {
    /// The default for fields is to require no configuration.
    #[default]
    None,
    /// Configuration for number fields.
    Number {
        /// Unsure what this means.
        fraction_digits: Option<usize>,
    },
    /// Configuration for currency fields.
    Currency {
        /// The name of the currency.
        currency: String,
        /// Unsure what this means.
        fraction_digits: Option<usize>,
    },
    /// A multiselect or single select field.
    Selections {
        /// The list of possible options.
        options: Vec<Selection>,
    },
    /// Configuration for timestamp fields.
    Timestamp {
        /// Whether a timestamp field should be a date or a datetime.
        include_time: bool,
    },
}

/// A single selection option for a multiselect/select field.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Selection {
    /// the id of this option
    id: Uuid,
    /// the text to show for this option
    name: String,
}

/// The enumeration of types that a field object might be.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FieldType {
    /// A date field.
    Date,
    /// A plain text field.
    Text,
    /// A number field.
    Number,
    /// A currency field.
    Currency,
    /// A single-select option field.
    Select,
    /// A multi-select option field.
    MultiSelect,
    /// Multiple collaborators?
    MultiCollaborator,
    /// A field showing who created something.
    CreatedBy,
    /// A field showing who last modified something.
    LastUpdatedBy,
    /// A timestamp field recording when an item was created.
    CreatedAt,
    /// A timestamp field record when an item was last modified.
    UpdatedAt,
}

impl FieldType {
    /// A convenience for checking if a field type has config.
    pub fn has_config(&self) -> bool {
        matches!(
            self,
            FieldType::Number
                | FieldType::Currency
                | FieldType::Select
                | FieldType::MultiSelect
                | FieldType::CreatedAt
                | FieldType::UpdatedAt
        )
    }
}

/// A Nuclino page, which might be either an "item" or a "collection".
/// An item is a normal wiki page with Markdown content. A collection is
/// a list of items. The enum provides implementations for the functions
/// its variants have in common, so you can avoid accessing the variant
/// data unless you need to.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "object", rename_all = "camelCase")]
pub enum Page {
    /// Regular wiki pages with markdown content.
    Item(Item),
    /// Collection pages, which contain only lists of other pages.
    Collection(Collection),
}

impl Page {
    /// The id of this page.
    pub fn id(&self) -> &Uuid {
        match self {
            Page::Item(v) => v.id(),
            Page::Collection(v) => v.id(),
        }
    }

    /// The workspace this page belongs to.
    pub fn workspace(&self) -> &Uuid {
        match self {
            Page::Item(v) => v.workspace(),
            Page::Collection(v) => v.workspace(),
        }
    }

    /// The url of this page.
    pub fn url(&self) -> &str {
        match self {
            Page::Item(v) => v.url(),
            Page::Collection(v) => v.url(),
        }
    }

    /// The title of this page.
    pub fn title(&self) -> &str {
        match self {
            Page::Item(v) => v.title(),
            Page::Collection(v) => v.title(),
        }
    }

    /// When this page was created, as an ISO-8601-formatted string.
    pub fn created(&self) -> &str {
        match self {
            Page::Item(v) => v.created(),
            Page::Collection(v) => v.created(),
        }
    }

    /// The id of the user who created this page.
    pub fn created_by(&self) -> &Uuid {
        match self {
            Page::Item(v) => v.created_by(),
            Page::Collection(v) => v.created_by(),
        }
    }

    /// When this page was last modified, as an ISO-8601-formatted string.
    pub fn modified(&self) -> &str {
        match self {
            Page::Item(v) => v.modified(),
            Page::Collection(v) => v.modified(),
        }
    }

    /// The id of the user who last modified this page.
    pub fn modified_by(&self) -> &Uuid {
        match self {
            Page::Item(v) => v.modified_by(),
            Page::Collection(v) => v.modified_by(),
        }
    }
}

/// A "collection" is a kind of page in the Nuclino wiki. Collections are lists of items
///  shown on a single page in the Nuclino wiki. Unlike "items", collections do not include
/// any markdown-formatted content.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    id: Uuid,
    workspace_id: Uuid,
    url: String,
    title: String,
    created_at: String, // date
    created_user_id: Uuid,
    last_updated_at: String, // date
    last_updated_user_id: Uuid,
    child_ids: Vec<Uuid>,
}

impl Collection {
    /// The ID of this collection.
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// The id of the workspace this collection belongs to.
    pub fn workspace(&self) -> &Uuid {
        &self.workspace_id
    }

    /// This collection's url.
    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    /// This collection's title.
    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    /// Ids of the child pages of this collection; that is, what the collection contains.
    pub fn children(&self) -> &[Uuid] {
        self.child_ids.as_slice()
    }

    /// When this collection was created, as a string in ISO-8601
    pub fn created(&self) -> &str {
        self.created_at.as_str()
    }

    /// The ID of the user who created this collection.
    pub fn created_by(&self) -> &Uuid {
        &self.created_user_id
    }

    /// The last-modified time as an ISO-8601 string.
    pub fn modified(&self) -> &str {
        self.last_updated_at.as_str()
    }

    /// The id of the user who last modified this item.
    pub fn modified_by(&self) -> &Uuid {
        &self.last_updated_user_id
    }
}

/// A Nuclino page with markdown content optionally included. The API refers
/// to this as an "item" to distinguish it from collections, which are lists of items.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    id: Uuid,
    workspace_id: Uuid,
    url: String,
    title: String,
    created_at: String, // date
    created_user_id: Uuid,
    last_updated_at: String, // date
    last_updated_user_id: Uuid,
    fields: HashMap<String, String>,
    content: Option<String>,
    content_meta: Meta,
    highlight: Option<String>,
}

impl Item {
    /// All directly-accessible items in in the Nuclino API have UUID ids.
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// The id of the workspace this item belongs to.
    pub fn workspace(&self) -> &Uuid {
        &self.workspace_id
    }

    /// This page's url.
    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    /// This page's title.
    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    /// The content of the item formatted as Markdown
    pub fn content(&self) -> Option<&String> {
        self.content.as_ref()
    }

    /// Additional items associated with this page, such as items, collections, or downloadable files.
    pub fn content_meta(&self) -> &Meta {
        &self.content_meta
    }

    /// An object mapping field names to field values (see the Field struct).
    pub fn field_values(&self) -> &HashMap<String, String> {
        &self.fields
    }

    /// When this page was created, as a string in ISO-8601
    pub fn created(&self) -> &str {
        self.created_at.as_str()
    }

    /// The id of the user who created this page.
    pub fn created_by(&self) -> &Uuid {
        &self.created_user_id
    }

    /// The last-modified time of this item as an ISO-8601 string.
    pub fn modified(&self) -> &str {
        self.last_updated_at.as_str()
    }

    /// The ID of the user who last modified this page.
    pub fn modified_by(&self) -> &Uuid {
        &self.last_updated_user_id
    }

    /// If this item is returned in a list of search results, the search string to highlight.
    pub fn highlight_text(&self) -> Option<&String> {
        self.highlight.as_ref()
    }
}

/// A structure holding metadata about a single item.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    /// An array of IDs of all the items and collections that appear inside the content.
    pub item_ids: Vec<Uuid>,
    /// An array of IDs of all the files that appear inside the content.
    pub file_ids: Vec<Uuid>,
}

/// A downloadable file object, associated with a regular wiki page.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    id: Uuid,
    item_id: Uuid,
    file_name: String,
    created_at: String, // date
    created_user_id: Uuid,
    download: DownloadInfo,
}

impl File {
    /// All directly-accessible items in in the Nuclino API have UUID ids.
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// I'm not sure what the item id is.
    pub fn item_id(&self) -> &Uuid {
        &self.item_id
    }

    /// The name of the file that can be downloaded.
    pub fn filename(&self) -> &str {
        self.file_name.as_str()
    }

    /// When this downloadable item was added to the wikie.
    pub fn created(&self) -> &str {
        self.created_at.as_str()
    }

    /// The ID of the user who added this downloadable item to the wiki.
    pub fn created_by(&self) -> &Uuid {
        &self.created_user_id
    }

    /// A link to download the file from, with its expiration time.
    pub fn download_info(&self) -> &DownloadInfo {
        &self.download
    }
}

/// Information required to download a file.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadInfo {
    /// Download URL to the file. This link is valid for 10 minutes after creation.
    pub url: String,
    /// An ISO-8601-formatted string representing when this download link expires.
    pub expires_at: String,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::response_types::*;

    // All test input comes from the example data in the Nuclino API documentation.

    #[test]
    fn user() {
        let input = r#"{
          "status": "success",
          "data": {
            "object": "user",
            "id": "9bff403a-6e0a-4f17-beac-c4333bd719b4",
            "firstName": "Thomas",
            "lastName": "Anderson",
            "email": "thomas@nuclino.com",
            "avatarUrl": "https://files.nuclino.com/avatars/9bff403a-6e0a-4f1..."
          }
        }"#;
        let result = serde_json::from_str::<Response<User>>(input)
            .expect("must be able to deserialize User response");
        assert!(result.is_success());
        let user = result.data().expect("we expected a valid user object.");
        assert_eq!(user.first_name, "Thomas".to_string());
        assert_eq!(user.first_name(), "Thomas");
        let id = Uuid::from_str("9bff403a-6e0a-4f17-beac-c4333bd719b4")
            .expect("expected a valid uuid in the example");
        assert_eq!(user.id(), &id);
    }

    #[test]
    fn workspace() {
        let input = r#"{
          "status": "success",
          "data": {
            "object": "workspace",
            "id": "127a8c4a-b3c6-4a42-8fef-b6c521e6c8cf",
            "teamId": "020f9737-7b21-442b-85eb-bd420e5593b2",
            "name": "General",
            "createdAt": "2021-12-15T15:54:23.598Z",
            "createdUserId": "2e96f3bb-c742-4164-af2c-151ab2fd346b",
            "fields": [
              {
                "object": "field",
                "id": "1504df6f-5704-43e9-9af9-79ed801828d8",
                "type": "date",
                "name": "My date field"
              }
            ],
            "childIds": ["aaf6d580-565d-497b-9ff3-b32075de3f4c"]
          }
        }"#;
        let result = serde_json::from_str::<Response<Workspace>>(input)
            .expect("must be able to deserialize Workspace response");
        assert!(result.is_success());

        let workspace = result.data().expect("we expected a valid workspace");
        let id = Uuid::from_str("127a8c4a-b3c6-4a42-8fef-b6c521e6c8cf")
            .expect("the example id should be a valid uuid");
        assert_eq!(workspace.id(), &id);
        let child_id =
            Uuid::from_str("aaf6d580-565d-497b-9ff3-b32075de3f4c").expect("expected valid uuid");
        assert!(workspace.children().contains(&child_id));
    }

    #[test]
    fn workspace_list() {
        let input = r#"{
          "status": "success",
          "data": {
            "object": "list",
            "results": [
              {
                "object": "workspace",
                "id": "127a8c4a-b3c6-4a42-8fef-b6c521e6c8cf",
                "teamId": "020f9737-7b21-442b-85eb-bd420e5593b2",
                "name": "General",
                "createdAt": "2021-12-15T15:54:23.598Z",
                "createdUserId": "2e96f3bb-c742-4164-af2c-151ab2fd346b",
                "fields": [
                  {
                    "object": "field",
                    "id": "1504df6f-5704-43e9-9af9-79ed801828d8",
                    "type": "date",
                    "name": "My date field"
                  }
                ],
                "childIds": ["aaf6d580-565d-497b-9ff3-b32075de3f4c"]
              },
              {
                "object": "workspace",
                "id": "66be346f-44e2-49da-888b-a2e381d4d92a",
                "teamId": "020f9737-7b21-442b-85eb-bd420e5593b2",
                "name": "Sprint planning",
                "createdAt": "2021-12-15T15:54:05.085Z",
                "createdUserId": "2e96f3bb-c742-4164-af2c-151ab2fd346b",
                "fields": [],
                "childIds": []
              }
            ]
          }
        }"#;
        let result = serde_json::from_str::<Response<List<Workspace>>>(input)
            .expect("must be able to deserialize Workspace response");
        assert!(result.is_success());
        let wrapper = result
            .data()
            .expect("successful deserializations should result in valid data. it's a fact.");
        assert_eq!(wrapper.slice().len(), 2);
    }

    #[test]
    fn team() {
        let input = r#"{
          "status": "success",
          "data": {
            "object": "team",
            "id": "020f9737-7b21-442b-85eb-bd420e5593b2",
            "url": "https://app.nuclino.com/Team-One",
            "name": "Team One",
            "createdAt": "2021-10-21T09:34:47.885Z",
            "createdUserId": "2e96f3bb-c742-4164-af2c-151ab2fd346b"
          }
        }"#;
        let result = serde_json::from_str::<Response<Team>>(input)
            .expect("must be able to deserialize team response");
        assert!(result.is_success());
        let team = result.data().expect("expected a valid team object");
        assert_eq!(team.name(), "Team One");
    }

    #[test]
    fn team_list() {
        let input = r#"{
          "status": "success",
          "data": {
            "object": "list",
            "results": [
              {
                "object": "team",
                "id": "020f9737-7b21-442b-85eb-bd420e5593b2",
                "url": "https://app.nuclino.com/Team-One",
                "name": "Team One",
                "createdAt": "2021-10-21T09:34:47.885Z",
                "createdUserId": "2e96f3bb-c742-4164-af2c-151ab2fd346b"
              },
              {
                "object": "team",
                "id": "2e5474ad-c433-4a02-9bde-5455a12d025f",
                "url": "https://app.nuclino.com/Team-Two",
                "name": "Team Two",
                "createdAt": "2021-11-29T14:21:30.052Z",
                "createdUserId": "2e96f3bb-c742-4164-af2c-151ab2fd346b"
              }
            ]
          }
        }"#;

        let result = serde_json::from_str::<Response<List<Team>>>(input)
            .expect("must be able to deserialize a list of teams");
        assert!(result.is_success());
    }

    #[test]
    fn file_objects() {
        let input = r#"{
          "status": "success",
          "data": {
            "object": "file",
            "id": "eec0a152-b1e9-43fd-bef8-987f95c85c6e",
            "itemId": "dd9a69db-048d-4644-8738-36bee31bbee0",
            "fileName": "screenshot.png",
            "createdAt": "2021-12-15T07:58:11.196Z",
            "createdUserId": "2e96f3bb-c742-4164-af2c-151ab2fd346b",
            "download": {
              "url": "https://nuclino-files.s3.eu-central-1.amazonaws.com/a122ab11...",
              "expiresAt": "2021-12-15T08:08:49.931Z"
            }
          }
        }"#;
        let result = serde_json::from_str::<Response<File>>(input)
            .expect("must be able to deserialize a file response");
        assert!(result.is_success());
    }

    #[test]
    fn item_list() {
        let input = r#"{
          "status": "success",
          "data": {
            "object": "list",
            "results": [
              {
                "object": "item",
                "id": "aaf6d580-565d-497b-9ff3-b32075de3f4c",
                "workspaceId": "127a8c4a-b3c6-4a42-8fef-b6c521e6c8cf",
                "url": "https://app.nuclino.com/t/b/aaf6d580-565d-497b-9ff3-b32075de3f4c",
                "title": "My Item",
                "createdAt": "2021-12-15T15:55:19.527Z",
                "createdUserId": "2e96f3bb-c742-4164-af2c-151ab2fd346b",
                "lastUpdatedAt": "2021-12-15T17:02:53.487Z",
                "lastUpdatedUserId": "2e96f3bb-c742-4164-af2c-151ab2fd346b",
                "fields": {
                  "My date field": "2025-01-20"
                },
                "contentMeta": { "itemIds": [], "fileIds": [] }
              },
              {
                "object": "collection",
                "id": "e9e648b3-8ce3-410d-8ef8-51b46c63cdaf",
                "workspaceId": "127a8c4a-b3c6-4a42-8fef-b6c521e6c8cf",
                "url": "https://app.nuclino.com/t/b/e9e648b3-8ce3-410d-8ef8-51b46c63cdaf",
                "title": "My collection",
                "createdAt": "2021-12-15T17:02:56.276Z",
                "createdUserId": "2e96f3bb-c742-4164-af2c-151ab2fd346b",
                "lastUpdatedAt": "2021-12-15T17:03:00.389Z",
                "lastUpdatedUserId": "2e96f3bb-c742-4164-af2c-151ab2fd346b",
                "childIds": []
              }
            ]
          }
        }"#;

        // The above response contains both items and collections, so we use serde's tagged enums
        // approach to deserialize. This puts some onus on the library user to match on the enum
        // to figure out which is which.
        let result = serde_json::from_str::<Response<List<Page>>>(input)
            .expect("must be able to deserialize a list of items response");
        assert!(result.is_success());
        let list = result
            .data()
            .expect("successful deserialization should give us data")
            .as_vec();
        assert_eq!(list.len(), 2);
    }
}
