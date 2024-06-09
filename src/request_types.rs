//! Types you'll use when sending new data to Nuclino.

use serde::Serialize;
use serde_with::skip_serializing_none;
use uuid::Uuid;

/// An enum used by NewPage to represent the kind of page being created.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum PageKind {
    /// Creating a regular wiki page.
    #[default]
    Item,
    /// Creating a collection.
    Collection,
}

/// Use this struct if you are creating new collections or pages from scratch.
/// Don't create one directly; use the NewPageBuilder instead.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewPage {
    /// The workspace where this page should be created. Mutually exclusive with parent_id.
    workspace_id: Option<Uuid>,
    /// The parent collection this page should be created within. Mutually exclusive with workspace_id.
    parent_id: Option<Uuid>,
    /// An optional page title.
    title: Option<String>,
    /// An optional index inside the parent to place this page. Defaults to the end of the list of children.
    index: Option<usize>,
    /// How Nuclino distinguishes collections from items.
    object: PageKind,
    /// Markdown-formatted content. Required for items; ignored for collections.
    content: Option<String>,
}

/// The builder pattern for new items and collections.
pub struct NewPageBuilder {
    workspace_id: Option<Uuid>,
    parent_id: Option<Uuid>,
    title: Option<String>,
    index: Option<usize>,
    object: PageKind,
    content: Option<String>,
}

impl NewPageBuilder {
    /// Start building a new item page.
    pub fn item() -> Self {
        Self {
            workspace_id: None,
            parent_id: None,
            title: None,
            index: None,
            object: PageKind::Item,
            content: None,
        }
    }

    /// Start building a new collection page.
    pub fn collection() -> Self {
        Self {
            workspace_id: None,
            parent_id: None,
            title: None,
            index: None,
            object: PageKind::Collection,
            content: None,
        }
    }

    /// Finalize your new page. Doesn't (yet) try to ensure you've set one of parent_id or workspace_id.
    pub fn build(&self) -> NewPage {
        let content = if matches!(self.object, PageKind::Collection) {
            None
        } else {
            self.content.clone()
        };
        NewPage {
            workspace_id: self.workspace_id,
            parent_id: self.parent_id,
            title: self.title.clone(),
            index: self.index,
            object: self.object.clone(),
            content,
        }
    }

    /// Set the title of the new page.
    pub fn title(&mut self, title: &str) -> &mut Self {
        self.title = Some(title.to_string());
        self
    }

    /// Choose where to create this page in a list of existing children of the page's intended parent.
    pub fn index(&mut self, index: usize) -> &mut Self {
        self.index = Some(index);
        self
    }

    /// Create this new page at the top level of the workspace with this id. Mutually exclusive with parent().
    pub fn workspace(&mut self, id: &Uuid) -> &mut Self {
        self.workspace_id = Some(*id);
        self.parent_id = None;
        self
    }

    /// Create this new page as a child of a specific parent page. Mutually exclusive with workspace().
    pub fn parent(&mut self, id: &Uuid) -> &mut Self {
        self.parent_id = Some(*id);
        self.workspace_id = None;
        self
    }

    /// Provide markdown-formatted content for this new page. Ignored for collections, even if
    /// set, because the Nuclino API will treat that as an error.
    pub fn content(&mut self, content: &str) -> &mut Self {
        self.content = Some(content.to_string());
        self
    }
}

/// This structure is used by the update endpoints for Items and Collections.
/// It's simple enough that you can create it directly.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyItem {
    /// Updated title. Optional.
    pub title: Option<String>,
    /// Updated content. Optional.
    pub content: Option<String>,
}

#[cfg(test)]
mod tests {
    use uuid::uuid;

    use super::NewPageBuilder;

    #[test]
    fn new_page_builder() {
        let parent = uuid!("e9e648b3-8ce3-410d-8ef8-51b46c63cdaf"); // from the Nuclino examples; is a collection
        let page = NewPageBuilder::item()
            .title("I am entitled")
            .content("This is *markdown*")
            .parent(&parent)
            .index(5)
            .build();
        assert_eq!(page.title, Some("I am entitled".to_string()));
        assert_eq!(page.content, Some("This is *markdown*".to_string()));
        assert_eq!(page.parent_id, Some(parent));
        assert!(page.workspace_id.is_none());
    }
}
