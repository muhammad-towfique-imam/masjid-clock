use serde::Serialize;

#[derive(Serialize)]
pub struct DisplayData {
    pub title: String,
    pub subtitle: String,
}
