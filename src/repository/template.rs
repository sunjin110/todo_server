use askama::{Template, Result};

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