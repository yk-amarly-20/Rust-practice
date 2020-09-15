// rendering

extern crate todo;
use todo::entities;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<entities::TodoEntry>
}
