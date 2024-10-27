use serde::Serialize;

#[derive(Serialize)]
pub struct Article {
    pub title: String,
    pub link: String,
}
