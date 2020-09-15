// rendering
use crate::entry::TodoEntry;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub entries: Vec<TodoEntry>
}
