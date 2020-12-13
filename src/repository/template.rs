use askama::{Template, Result};
use serde::Deserialize;

pub struct TodoEntry {
    pub id: u32,
    pub text: String,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub entries: Vec<TodoEntry>,
}

pub fn render(t: IndexTemplate) -> Result<String> {
    t.render()
}

#[derive(Deserialize)]
pub struct AddParams {
    pub text: String,
}

#[derive(Deserialize)]
pub struct DeleteParams {
    pub id: u32,
}
